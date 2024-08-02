use std::{
    cell::{Cell, RefCell},
    fmt::Display,
};

use lazy_regex::{regex, regex_is_match};

/// The full type name including namespaces and template types. ie `std::vector<int>`
#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Symbol(String, String),
    String(String),
    None,
}

/// A repersentation of a c++ typename. For example: `std::vector<int>`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    name: Type,
    pointer_count: Cell<usize>,
    modifiers: RefCell<String>,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match &self.name {
            Type::Symbol(s, _) => s,
            Type::String(s) => s,
            Type::None => NONETYPE_ERROR,
        };
        write!(f, "{}", s)
    }
}

const NONETYPE_ERROR: &str = "/* error: attempt to display invalid type */";

// This impl block contains all the `static` functions on the Symbol class.
impl Symbol {
    const fn _new(name: Type) -> Self {
        Self {
            name,
            pointer_count: Cell::new(0),
            modifiers: RefCell::new(String::new()),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn new(name: String) -> Self {
        let unlambdad = Self::replace_unnamed_types(&name);
        let without_templates = Self::remove_template_types(&unlambdad);
        Self::_new(Type::Symbol(without_templates, unlambdad))
    }

    pub const fn from_string(name: String) -> Self {
        Self::_new(Type::String(name))
    }

    pub const fn none() -> Self {
        Self::_new(Type::None)
    }

    pub fn replace_unnamed_types(name: &str) -> String {
        let name = name.replace("<unnamed-tag>", "unnamed_tag");
        let name = regex!(r"<lambda_(\w+?)>")
            .replace_all(&name, "lambda_$1")
            .into_owned();

        regex!(r"<unnamed-(type|enum)-(.+?)>")
            .replace_all(&name, |captures: &regex::Captures| {
                captures
                    .get(2)
                    .expect("Compiled regex will always have a capture group")
                    .as_str()
                    .to_owned()
            })
            .into_owned()
    }

    pub fn remove_template_types(name: &str) -> String {
        let mut result = String::new();
        let mut template_depth = 0;
        for ch in name.chars() {
            match ch {
                '<' => {
                    template_depth += 1;
                }
                '>' => {
                    template_depth -= 1;
                }
                _ => {
                    if template_depth == 0 {
                        result.push(ch);
                    }
                }
            }
        }

        assert!(!result.ends_with("::"));
        result
    }
}

// This impl block contains all the instance functions on the Symbol class.
impl Symbol {
    pub fn increment_pointer_count(&self) {
        self.pointer_count.set(self.pointer_count.get() + 1);
    }

    pub fn add_modifiers(&self, modifiers: &str) {
        self.modifiers.borrow_mut().push_str(modifiers);
    }

    pub fn fully_qualifed(&self) -> String {
        match &self.name {
            Type::Symbol(_, s) => {
                let pointers = self.pointer_count.get();
                let pointers = "*".repeat(pointers);
                let modifiers = &self.modifiers.borrow();

                format!("{modifiers}{s}{pointers}")
            }
            Type::String(s) => s.to_string(),
            Type::None => NONETYPE_ERROR.to_string(),
        }
    }

    pub fn namespace_vec(&self) -> Vec<String> {
        let s = self.to_string();

        let mut result = Vec::new();
        let mut current = String::new();
        let mut template_depth = 0;

        let chars: Vec<char> = s.chars().collect();
        let mut skip = false;

        for (i, char) in chars.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            match char {
                '<' => {
                    template_depth += 1;
                    current.push('<');
                }
                '>' => {
                    template_depth -= 1;
                    current.push('>');
                }
                ':' if template_depth == 0 && i + 1 < chars.len() && chars[i + 1] == ':' => {
                    result.push(current.trim().to_string());
                    current.clear();
                    skip = true; // skip the next ':'
                }
                _ => {
                    current.push(chars[i]);
                }
            }
        }

        if !current.is_empty() {
            result.push(current.trim().to_string());
        }

        result
    }

    /// We need to split template types by commas, however sometimes nested templates contain commas inside <>.
    /// This is a special implementation of the `split()` function to handle the above case.
    fn templates_vec(&self) -> Vec<String> {
        let Type::Symbol(_, type_name) = &self.name else {
            return Vec::new();
        };

        let mut result = Vec::new();
        let mut current = String::new();
        let mut template_depth = 0;

        for char in type_name.chars().skip(type_name.find("::").unwrap_or(0)) {
            match char {
                '<' | '(' => {
                    if template_depth != 0 {current.push(char);}
                    template_depth += 1;
                }
                '>' | ')' => {
                    template_depth -= 1;
                    if template_depth < 0 {
                        template_depth = 0;
                    }
                    if template_depth != 0 {current.push(char);}
                }
                ',' if template_depth == 1 => {
                    result.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    if template_depth != 0 {current.push(char);}
                }
            }
        }

        if !current.is_empty() {
            result.push(current.trim().to_string());
        }

        result
    }

    /// Figures out the template types based on the class name stored in the PDB.
    /// For example `my_namespace::MyClass<int, std::string>` would return:
    /// `[("typename", "T", "int"), ("typename", "U", std::string)]`
    pub fn templates_by_type(&self) -> Vec<(String, String, String)> {
        /// Converts a number into base 7 where each digit is a letter from T, U, V, W, X, Y, Z.
        fn number_to_template_type_name(i: usize) -> String {
            match i {
                0 => "T".to_string(),
                1 => "U".to_string(),
                2 => "V".to_string(),
                3 => "W".to_string(),
                4 => "X".to_string(),
                5 => "Y".to_string(),
                6 => "Z".to_string(),
                _ => {
                    let remainder = i % 7;
                    let i = i / 7;
                    let mut s = number_to_template_type_name(i);
                    s.push_str(&number_to_template_type_name(remainder));
                    s
                }
            }
        }

        self.templates_vec()
            .into_iter()
            .enumerate()
            .map(|(i, template)| {
                let template_keyword = if regex_is_match!(r"^\d+\.\d+$", &template) {
                    "double"
                } else if regex_is_match!(r"^\d+$", &template) {
                    "long long"
                } else {
                    "typename"
                }
                .to_string();

                let identifier = number_to_template_type_name(i);

                (template_keyword, identifier, template)
            })
            .collect()
    }

    /// Returns a string that would be used on this class to define its template classes.
    /// For example `template <typename T, typename U>`, `template<typename T, int L>`, `template<typename T, template<typename U, int I> class Arr>`, or `template <typename... Arguments>`.
    /// Disclaimer: I do not support this nonsense: `template<class T, T::type n = 0>`.
    pub fn templates(templates_by_type: &[(String, String, String)]) -> String {
        let mut result = String::new();

        if templates_by_type.is_empty() {
            return result;
        }

        result.push_str("template <");
        for (i, (template_keyword, identifier, _)) in templates_by_type.iter().enumerate() {
            if i != 0 {
                result.push_str(", ");
            }

            result.push_str(template_keyword);
            result.push(' ');
            result.push_str(identifier);
        }

        result.push_str("> ");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_templates() {
        let symbol = Symbol::new("my_namespace::MyClass<int, std::string>".to_string());
        let templates = symbol.templates_vec();
        assert_eq!(templates, vec!["int", "std::string"]);
        let templates = symbol.templates_by_type();
        assert_eq!(
            templates,
            vec![
                ("typename".to_string(), "T".to_string(), "int".to_string()),
                (
                    "typename".to_string(),
                    "U".to_string(),
                    "std::string".to_string()
                )
            ]
        );

        let symbol = Symbol::new("my_namespace::MyClass<int, std::string, 10.5>".to_string());
        let templates = symbol.templates_by_type();
        assert_eq!(
            templates,
            vec![
                ("typename".to_string(), "T".to_string(), "int".to_string()),
                (
                    "typename".to_string(),
                    "U".to_string(),
                    "std::string".to_string()
                ),
                ("double".to_string(), "V".to_string(), "10.5".to_string())
            ]
        );

        let symbol = Symbol::new("my_namespace::MyClass<int, std::string, 10.5, 8>".to_string());
        let templates = symbol.templates_by_type();
        assert_eq!(
            templates,
            vec![
                ("typename".to_string(), "T".to_string(), "int".to_string()),
                (
                    "typename".to_string(),
                    "U".to_string(),
                    "std::string".to_string()
                ),
                ("double".to_string(), "V".to_string(), "10.5".to_string()),
                ("long long".to_string(), "W".to_string(), "8".to_string())
            ]
        );

        let symbol = Symbol::new(
            "my_namespace::MyClass<my_namespace::MyClass<int, std::string>, 7.999>".to_string(),
        );
        let templates = symbol.templates_by_type();
        assert_eq!(
            templates,
            vec![
                (
                    "typename".to_string(),
                    "T".to_string(),
                    "my_namespace::MyClass<int, std::string>".to_string()
                ),
                ("double".to_string(), "U".to_string(), "7.999".to_string())
            ]
        );
    }

    #[test]
    fn test_name_parsing() {
        let symbol = Symbol::new("my_namespace::MyClass<int, std::string<string>>".to_string());
        symbol.increment_pointer_count();
        let namespace = symbol.namespace_vec();
        assert_eq!(
            namespace,
            vec!["my_namespace".to_string(), "MyClass".to_string()]
        );
        assert_eq!(symbol.to_string(), "my_namespace::MyClass".to_string());
        assert_eq!(
            symbol.fully_qualifed(),
            "my_namespace::MyClass<int, std::string<string>>*"
        );
        assert_eq!(
            symbol.templates_by_type(),
            vec![
                ("typename".to_string(), "T".to_string(), "int".to_string()),
                (
                    "typename".to_string(),
                    "U".to_string(),
                    "std::string<string>".to_string()
                )
            ]
        );

        let symbol = Symbol::new("my_namespace<int, std::string, 10.5>::MyClass".to_string());
        let namespace = symbol.namespace_vec();
        assert_eq!(
            namespace,
            vec!["my_namespace".to_string(), "MyClass".to_string()]
        );
        assert_eq!(symbol.to_string(), "my_namespace::MyClass".to_string());
        assert_eq!(symbol.fully_qualifed(), "my_namespace<int, std::string, 10.5>::MyClass");
        assert_eq!(symbol.templates_by_type().len(), 0);
    }
}
