use anyhow::{bail, Result};
use core::panic;
use lazy_regex::regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use topo_sort::{SortResults, TopoSort};

use pdb::FallibleIterator;

fn primitive_name(data: pdb::PrimitiveType) -> String {
    #[allow(clippy::match_same_arms)]
    let mut name = match data.kind {
        pdb::PrimitiveKind::Void => "void",
        pdb::PrimitiveKind::Char => "char",
        pdb::PrimitiveKind::UChar => "unsigned char",
        pdb::PrimitiveKind::I8 => "int8_t",
        pdb::PrimitiveKind::U8 => "uint8_t",
        pdb::PrimitiveKind::I16 => "int16_t",
        pdb::PrimitiveKind::U16 => "uint16_t",
        pdb::PrimitiveKind::I32 => "int32_t",
        pdb::PrimitiveKind::U32 => "uint32_t",
        pdb::PrimitiveKind::I64 => "int64_t",
        pdb::PrimitiveKind::U64 => "uint64_t",
        pdb::PrimitiveKind::F32 => "float",
        pdb::PrimitiveKind::F64 => "double",
        pdb::PrimitiveKind::Bool8 => "bool",

        pdb::PrimitiveKind::NoType => "void",
        pdb::PrimitiveKind::RChar => "char", // Assuming RChar is regular char
        pdb::PrimitiveKind::WChar => "wchar_t",
        pdb::PrimitiveKind::RChar16 => "char16_t",
        pdb::PrimitiveKind::RChar32 => "char32_t",
        pdb::PrimitiveKind::Short => "short",
        pdb::PrimitiveKind::UShort => "unsigned short",
        pdb::PrimitiveKind::Long => "long",
        pdb::PrimitiveKind::ULong => "unsigned long",
        pdb::PrimitiveKind::Quad => "int64_t", // Quad is typically 64-bit
        pdb::PrimitiveKind::UQuad => "uint64_t",
        pdb::PrimitiveKind::Octa => "int128_t", // Assuming Octa is 128-bit
        pdb::PrimitiveKind::UOcta => "uint128_t",
        pdb::PrimitiveKind::I128 => "int128_t",
        pdb::PrimitiveKind::U128 => "uint128_t",
        pdb::PrimitiveKind::F16 => "__fp16", // Assuming F16 is half-precision floating-point
        pdb::PrimitiveKind::F32PP => "float _Complex", // Assuming F32PP is complex float
        pdb::PrimitiveKind::F48 => "float _Complex", // Assuming F48 is complex float (needs confirmation)
        pdb::PrimitiveKind::F80 => "long double",
        pdb::PrimitiveKind::F128 => "__float128", // Quad-precision floating-point
        pdb::PrimitiveKind::Complex32 => "_Complex float",
        pdb::PrimitiveKind::Complex64 => "_Complex double",
        pdb::PrimitiveKind::Complex80 => "_Complex long double",
        pdb::PrimitiveKind::Complex128 => "_Complex __float128", // Assuming complex quad-precision
        pdb::PrimitiveKind::Bool16 => "_Bool16",                 // Assuming 16-bit boolean
        pdb::PrimitiveKind::Bool32 => "_Bool32",                 // Assuming 32-bit boolean
        pdb::PrimitiveKind::Bool64 => "_Bool64",                 // Assuming 64-bit boolean
        pdb::PrimitiveKind::HRESULT => "HRESULT",
        _ => panic!("Unknown primitive type {:?}", data.kind),
    }
    .to_string();

    if data.indirection.is_some() {
        name.push('*');
    }

    name
}

struct FunctionAttributes(pdb::FunctionAttributes);

impl FunctionAttributes {
    fn calling_convention(&self) -> String {
        let calling_convention = CallingConvention(self.0.calling_convention());
        calling_convention.as_cpp().to_string()
    }

    fn is_constructor(&self) -> bool {
        self.0.is_constructor()
    }
}

struct FieldAttributes {
    attributes: pdb::FieldAttributes,
    is_virtual: bool,
}

impl FieldAttributes {
    fn as_string(&self) -> String {
        let mut s = match self.attributes.access() {
            1 => "private: ",
            2 => "protected: ",
            3 => "public: ",
            _ => "",
        }
        .to_string();

        if self.attributes.is_static() {
            s = format!("{s}static ");
        }
        if self.is_virtual || self.attributes.is_virtual() || self.attributes.is_pure_virtual() {
            s = format!("{s}virtual ");
        }
        s
    }
}

fn stringify_properties(properties: pdb::TypeProperties) -> String {
    let mut s = String::new();

    if properties.packed() {
        s.push_str("packed ");
    }
    if properties.overloaded_operators() {
        s.push_str("overloaded_operators ");
    }
    if properties.is_nested_type() {
        s.push_str("is_nested_type ");
    }
    if properties.contains_nested_types() {
        s.push_str("contains_nested_types ");
    }
    if properties.overloaded_assignment() {
        s.push_str("overloaded_assignment ");
    }
    if properties.overloaded_casting() {
        s.push_str("overloaded_casting ");
    }
    if properties.scoped_definition() {
        s.push_str("scoped_definition ");
    }
    if properties.sealed() {
        s.push_str("sealed ");
    }
    if properties.intrinsic_type() {
        s.push_str("intrinsic_type ");
    }
    if properties.hfa() != 0 {
        s.push_str(&format!("hfa={} ", properties.hfa()));
    }

    if s.is_empty() {
        s
    } else {
        format!("/* {s}*/ ")
    }
}

fn list_namespaces_as_vec(class_name: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut template_depth = 0;

    let chars: Vec<char> = class_name.chars().collect();
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
            ':' if i + 1 < chars.len() && chars[i + 1] == ':' => {
                if template_depth == 0 {
                    result.push(current.trim().to_string());
                    current.clear();
                    skip = true; // skip the next ':'
                } else {
                    current.push(':');
                }
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

fn find_type<'a>(
    type_finder: &pdb::TypeFinder<'a>,
    type_index: pdb::TypeIndex,
) -> Result<pdb::TypeData<'a>> {
    Ok(type_finder.find(type_index)?.parse()?)
}

struct DecompilationResult<'a> {
    /// A multi-line string repersenting the c++ code that would generate this type
    repersentation: String,
    /// The full type name including namespaces and template types. ie `std::vector<int>`
    name: String,
    /// An interior-mutable set of all the types that this type depends on. Used for topological sorting.
    /// This is required becuase clang expects all types to be defined before they are used.
    dependencies: RefCell<HashSet<String>>,
    /// The type data from pdb crate that this result is based on.
    /// This is an optional in order to handle pdb parsing errors.
    data: Option<pdb::TypeData<'a>>,
    /// A borrowed refrence to a pdb crate type finder.
    /// This type finder is shared across all `DecompilationResult` for a certian pdb file.
    type_finder: &'a pdb::TypeFinder<'a>,
    /// An interior-mutable set of all the base classes that this class inherits from.
    base_classes: RefCell<HashSet<String>>,
    /// Used for array sizes. This data must be specfied after a type's name. ie `int my_int[4]` not `int[4] my_int`
    name_suffix: String,
}

fn number_to_method_arg_name(i: usize) -> String {
    match i {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        4 => "e".to_string(),
        5 => "f".to_string(),
        6 => "g".to_string(),
        _ => {
            let remainder = i % 7;
            let i = i / 7;
            let mut s = number_to_method_arg_name(i);
            s.push_str(&number_to_method_arg_name(remainder));
            s
        }
    }
}

/// Contains array dimensions as specified in the PDB. This is not what you expect:
///
/// * Dimensions are specified in terms of byte sizes, not element counts.
/// * Multidimensional arrays aggregate the lower dimensions into the sizes of the higher
///   dimensions.
///
/// Thus a `float[4][4]` has `dimensions: [16, 64]`. Determining array dimensions in terms
/// of element counts requires determining the size of the `element_type` and iteratively
/// dividing.
fn array_dimensions(data: &pdb::ArrayType) -> String {
    let mut dimensions = Vec::new();
    let mut size = 1;
    for dimension in data.dimensions.iter().rev() {
        size *= dimension;
        dimensions.push(size.to_string());
    }
    dimensions.reverse();
    format!("[{}]", dimensions.join("]["))
}

impl<'a> DecompilationResult<'a> {
    /// Constructor for decompilation result based on a `pdb::TypeData`.
    fn from_data(
        parent: Option<&Self>,
        type_finder: &'a pdb::TypeFinder<'a>,
        data: pdb::TypeData<'a>,
    ) -> Self {
        let mut dc = DecompilationResult {
            repersentation: String::new(),
            name: String::new(),
            dependencies: RefCell::new(HashSet::new()),
            data: Some(data),
            type_finder,
            base_classes: RefCell::new(HashSet::new()),
            name_suffix: String::new(),
        };

        dc.calculate_name();
        if let Some(parent) = parent {
            parent.drain_dependencies(&dc);
        }

        dc
    }

    /// Constructor for decompilation result based on a `pdb::TypeIndex`.
    fn from_index(
        parent: Option<&Self>,
        type_finder: &'a pdb::TypeFinder<'a>,
        type_index: pdb::TypeIndex,
    ) -> Self {
        match find_type(type_finder, type_index) {
            Ok(data) => DecompilationResult::from_data(parent, type_finder, data),
            Err(e) => DecompilationResult {
                repersentation: format!("/* error processing type index {type_index} {e}*/"),
                name: format!("/* error processing type index {type_index} {e}*/"),
                dependencies: RefCell::new(HashSet::new()),
                data: None,
                type_finder,
                base_classes: RefCell::new(HashSet::new()),
                name_suffix: String::new(),
            },
        }
    }

    /// invariant: the data parameter to this function must equal self.data
    /// this invariant exists in order to enforce the type of the data parameter by the caller
    fn function_pointer_to_string(&self, data: &pdb::ProcedureType) -> (String, String) {
        // todo! actually check the invariant

        let function_attributes = FunctionAttributes(data.attributes);
        // function pointers are never constructors so that case is unchecked
        let calling_convention = function_attributes.calling_convention();

        let return_type = data.return_type.map_or_else(
            || "void".to_string(),
            |return_type| {
                let dc = DecompilationResult::from_index(Some(self), self.type_finder, return_type);
                dc.name
            },
        );

        let dc = DecompilationResult::from_index(Some(self), self.type_finder, data.argument_list);
        let arguments = &dc.repersentation;

        (
            format!("{return_type}({calling_convention}*"),
            format!(")({arguments})"),
        )
    }

    fn calculate_name(&mut self) {
        let type_finder = self.type_finder;

        self.name = replace_unnamed_types(&match &self.data {
            Some(pdb::TypeData::Pointer(data)) => {
                // todo! handle data.pointerAttributes
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.underlying_type);
                if let Some(pdb::TypeData::Procedure(data)) = dc.data {
                    let (name, suffix) = dc.function_pointer_to_string(&data);
                    self.name_suffix = suffix;
                    name
                } else {
                    format!("{}*", dc.name)
                }
            }

            Some(pdb::TypeData::Modifier(data)) => {
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.underlying_type);
                format!("{}{}", modifier_string(*data), dc.name)
            }

            Some(data) if data.name().is_some() => data
                .name()
                .expect("We already checked is_some")
                .to_string()
                .into_owned(),

            _ => {
                self.calculate_repersentation();
                self.name = self.repersentation.clone();
                return;
            }
        });

        self.calculate_repersentation();
    }

    #[allow(clippy::too_many_lines)]
    fn calculate_repersentation(&mut self) {
        let type_finder = self.type_finder;

        self.repersentation = match &self.data {
            Some(pdb::TypeData::Member(data)) => {
                // A field inside a class
                let dc = DecompilationResult::from_index(Some(self), type_finder, data.field_type);
                let offset = data.offset;
                let field_name = data.name.to_string();
                self.dependencies
                    .get_mut()
                    .insert(dc.raw_name().to_string());
                format!(
                    "/* offset {offset:3} */ {} {field_name}{}",
                    dc.name, dc.name_suffix
                )
            }
            Some(pdb::TypeData::StaticMember(data)) => {
                let dc = DecompilationResult::from_index(Some(self), type_finder, data.field_type);
                format!("static {}", dc.repersentation)
            }
            Some(pdb::TypeData::BaseClass(data)) => {
                let attributes = FieldAttributes {
                    attributes: data.attributes,
                    is_virtual: false,
                }
                .as_string();
                let dc = DecompilationResult::from_index(Some(self), type_finder, data.base_class);
                let raw_name = dc.raw_name().to_string();
                let type_name = dc.name;

                self.base_classes
                    .get_mut()
                    .insert(format!("{attributes}{type_name}"));
                self.dependencies.get_mut().insert(raw_name);

                format!(
                    "/* offset {:3} */ /* fields for {} */",
                    data.offset, type_name
                )
            }
            Some(pdb::TypeData::Primitive(data)) => primitive_name(*data), // A primitive type in c++. See primitive_name() for a full list.
            Some(pdb::TypeData::Class(_)) => {
                // A class type in c++
                // TODO handle data.properties
                self.class_to_string()
            }
            Some(pdb::TypeData::FieldList(data)) => {
                // A list of fields inside a class
                let mut fields: Vec<String> = Vec::new();
                let mut is_enumerate = false;
                for field in data.fields.clone() {
                    is_enumerate = matches!(field, pdb::TypeData::Enumerate(_));
                    let dc = DecompilationResult::from_data(Some(self), type_finder, field);
                    let mut s = dc.repersentation.to_string();

                    if !is_enumerate {
                        // if we dont have an enum, we need to add a semicolon
                        s.push(';');
                    }
                    fields.push(s);
                }
                if is_enumerate {
                    // if the last field in the field list is an enum, we must delete the last comma
                    fields.last_mut().map(std::string::String::pop);
                }
                if let Some(continuation) = data.continuation {
                    // recurse
                    let dc = DecompilationResult::from_index(Some(self), type_finder, continuation);
                    fields.push(format!("{};", dc.repersentation));
                }
                fields.join("\n")
            }
            Some(pdb::TypeData::Union(data)) => {
                // A union type in c++
                // TODO handle data.properties
                let type_name = data.name.to_string().into_owned();
                let (templates, type_name, templates_map) = parse_template_types(&type_name);

                if data.properties.forward_reference() {
                    self.repersentation = format!("union {type_name}");
                    return;
                }

                let dc = DecompilationResult::from_index(Some(self), type_finder, data.fields);
                let mut fields = do_indent(&dc.repersentation);

                for (identifier, template) in templates_map {
                    fields = fields.replace(&template, &identifier);
                }

                format!(
                    "{}{templates}union {type_name} {{\n{fields}\n}}",
                    stringify_properties(data.properties),
                )
            }
            Some(pdb::TypeData::Enumerate(data)) => {
                // One of the values inside an enum
                format!("{} = {},", data.name, data.value)
            }
            Some(pdb::TypeData::Enumeration(data)) => {
                // An enum type in c++
                // TODO handle data.properties
                let mut type_name = data.name.to_string().into_owned();

                if data.properties.forward_reference() {
                    self.repersentation = format!("enum {type_name}");
                    return;
                }

                let dc = DecompilationResult::from_index(Some(self), type_finder, data.fields);
                let fields = do_indent(&dc.repersentation);
                let underlying_dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.underlying_type);
                let underlying_type = underlying_dc.repersentation;

                if data.properties.is_nested_type() {
                    type_name = type_name.split("::").last().unwrap_or(&type_name).to_string();
                }
                format!(
                    "{}enum class {type_name} : {underlying_type} {{\n{fields}\n}};",
                    stringify_properties(data.properties),
                )
            }
            Some(pdb::TypeData::Pointer(data)) => {
                // Pointer to a diffrent datatype
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.underlying_type);
                format!("{}*", dc.repersentation)
            }
            Some(pdb::TypeData::Modifier(data)) => {
                // Wrapper around another type that describes a modifier. Can be const, volatile, or unaligned.
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.underlying_type);
                format!("{}{}", modifier_string(*data), dc.repersentation)
            }
            Some(pdb::TypeData::Array(data)) => {
                // I believe we can safely ignore the data.indexing_type field.
                let dimensions = array_dimensions(data);
                self.name_suffix = dimensions;
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.element_type);
                match data.stride {
                    Some(stride) => format!("/* stride {stride:3} */ {}", dc.repersentation),
                    None => dc.repersentation,
                }
            }
            Some(pdb::TypeData::Procedure(data)) => {
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.argument_list);
                let args = dc.repersentation;

                let return_type = data.return_type.map_or_else(
                    || "void".to_string(),
                    |return_type| {
                        let dc =
                            DecompilationResult::from_index(Some(self), type_finder, return_type);
                        dc.name
                    },
                );
                format!("{return_type}({args})")
            }
            Some(pdb::TypeData::Nested(data)) => {
                let dc = DecompilationResult::from_index(Some(self), type_finder, data.nested_type);
                format!(
                    "{}{}",
                    FieldAttributes {
                        attributes: data.attributes,
                        is_virtual: false
                    }
                    .as_string(),
                    dc.repersentation
                )
            }
            Some(pdb::TypeData::Method(data)) => {
                let dc = DecompilationResult::from_index(Some(self), type_finder, data.method_type);
                dc.method_to_string(&data.name.to_string(), Some(data.attributes))
            }
            Some(pdb::TypeData::OverloadedMethod(data)) => {
                // this just means we have more than one method with the same name
                // find the method list
                match find_type(type_finder, data.method_list) {
                    Ok(pdb::TypeData::MethodList(method_list)) => {
                        let mut s = Vec::new();
                        for pdb::MethodListEntry {
                            attributes,
                            method_type,
                            ..
                        } in method_list.methods
                        {
                            // hooray
                            let dc = DecompilationResult::from_index(
                                Some(self),
                                type_finder,
                                method_type,
                            );
                            s.push(dc.method_to_string(&data.name.to_string(), Some(attributes)));
                        }
                        s.join(";\n")
                    }
                    other => {
                        format!("processing OverloadedMethod, expected MethodList, got {other:?}.")
                    }
                }
            }
            Some(pdb::TypeData::MemberFunction(_)) => self.method_to_string("", None),
            Some(pdb::TypeData::VirtualBaseClass(data)) => {
                format!("todo! VirtualBaseClass {data:?}")
            }
            Some(pdb::TypeData::VirtualFunctionTablePointer(data)) => {
                let dc = DecompilationResult::from_index(Some(self), type_finder, data.table);
                format!("{}*", dc.repersentation)
            }
            Some(pdb::TypeData::Bitfield(data)) => {
                let dc =
                    DecompilationResult::from_index(Some(self), type_finder, data.underlying_type);
                self.name_suffix = format!(" : {}", data.length);
                format!("/* position {:3} */ {}", data.position, dc.repersentation)
            }
            Some(pdb::TypeData::ArgumentList(data)) => {
                let mut args = String::new();
                for (i, arg_type) in data.arguments.clone().into_iter().enumerate() {
                    let dc = DecompilationResult::from_index(Some(self), type_finder, arg_type);
                    args.push_str(&dc.name);
                    args.push(' ');
                    args.push_str(&number_to_method_arg_name(i));
                    args.push_str(&dc.name_suffix);
                    args.push(',');
                }
                args.pop(); // remove the last comma
                args
            }
            Some(pdb::TypeData::MethodList(data)) => format!("todo! MethodList {data:?}"),
            _ => todo!(),
        };
    }

    fn raw_name(&self) -> &str {
        let name: &str = &self.name;
        let index = name.find('<');
        index.map_or_else(|| name, |index| &name[..index])
    }

    fn class_to_string(&self) -> String {
        let name = &self.name;
        let data = self.data.as_ref();

        let Some(pdb::TypeData::Class(data)) = data else {
            return format!("/* error processing class {name} */");
        };

        let (templates, name, templates_map) = parse_template_types(name);
        let kind = match data.kind {
            pdb::ClassKind::Class => "class",
            pdb::ClassKind::Struct => "struct",
            pdb::ClassKind::Interface => "interface", // when can this happen?
        };

        if data.properties.forward_reference() {
            return format!("{templates}{kind} {name}");
        }

        let mut fields: Vec<String> = Vec::new();

        if data.derived_from.is_some() {
            todo!();
        }

        if let Some(type_index) = data.fields {
            let dc = DecompilationResult::from_index(Some(self), self.type_finder, type_index);
            fields.push(dc.repersentation);
        }

        let mut base_classes = String::new();
        for (i, base) in self.base_classes.borrow().iter().enumerate() {
            let prefix = if i == 0 { ':' } else { ',' };
            base_classes.push_str(&format!("{prefix} {base}"));
        }

        let mut fields = do_indent(&fields.join("\n"));
        for (identifier, template) in templates_map {
            fields = fields.replace(&template, &identifier);
            base_classes = base_classes.replace(&template, &identifier);
        }

        format!(
            "{}{templates}{kind} {name}{base_classes} {{\n{fields}\n}}",
            stringify_properties(data.properties)
        )
    }

    fn drain_dependencies(&self, other: &Self) {
        self.dependencies
            .borrow_mut()
            .extend(other.dependencies.borrow_mut().drain());
        self.base_classes
            .borrow_mut()
            .extend(other.base_classes.borrow_mut().drain());
    }

    fn method_to_string(
        &self,
        method_name: &str,
        field_attributes: Option<pdb::FieldAttributes>,
    ) -> String {
        let Some(pdb::TypeData::MemberFunction(data)) = self.data else {
            return format!("/* error processing method {method_name} */");
        };

        let function_attributes = FunctionAttributes(data.attributes);

        let type_finder = self.type_finder;
        let is_destructor = method_name.starts_with('~');

        let return_type = data.return_type;
        let return_type = if is_destructor || function_attributes.is_constructor() {
            ""
        } else {
            let dc = DecompilationResult::from_index(Some(self), type_finder, return_type);
            &format!("{} ", dc.name)
        };

        let calling_convention = function_attributes.calling_convention();
        let field_attributes = field_attributes.map_or_else(String::new, |field_attributes| {
            FieldAttributes {
                attributes: field_attributes,
                is_virtual: false,
            }
            .as_string()
        });

        let dc = DecompilationResult::from_index(Some(self), type_finder, data.argument_list);
        let arguments = dc.repersentation;

        format!("/* offset {:3} */ {field_attributes}{return_type}{calling_convention}{method_name}({arguments})", data.this_adjustment)
    }

    fn namespaced_repersentation(&self) -> String {
        let name = self.raw_name();

        let mut namespaces = list_namespaces_as_vec(name);
        let short_name = namespaces.pop().unwrap_or_default();
        let repersentation = &self.repersentation;
        if namespaces.is_empty() {
            return format!("{repersentation};");
        }

        let data_structure = repersentation.replacen(name, &short_name, 1);

        let mut namespaced_string = String::new();
        for namespace in &namespaces {
            namespaced_string.push_str("namespace ");
            namespaced_string.push_str(namespace);
            namespaced_string.push_str(" { ");
        }
        namespaced_string.push_str(&data_structure);
        namespaced_string.push(';');
        for _ in &namespaces {
            namespaced_string.push_str(" }");
        }
        namespaced_string.push(';');
        namespaced_string
    }
}

/// We need to split template types by commas, however sometimes nested templates contain commas inside <>.
/// This is a special implementation of the `split()` function to handle the above case.
fn split_templates(templates: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut template_depth = 0;

    let chars: Vec<char> = templates.chars().collect();

    for char in chars {
        match char {
            '<' | '(' => {
                template_depth += 1;
                current.push(char);
            }
            '>' | ')' => {
                template_depth -= 1;
                current.push(char);
            }
            ',' if template_depth == 0 => {
                result.push(current);
                current = String::with_capacity(32);
            }
            _ => {
                current.push(char);
            }
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn parse_template_types(class_name: &str) -> (String, String, HashMap<String, String>) {
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

    let re = regex!(r"(.+?)<(.*)>");
    let Some(captures) = re.captures(class_name) else {
        return (String::new(), class_name.to_string(), HashMap::new());
    };

    let class_name_without_templates = captures
        .get(1)
        .expect("Static regex always has one group")
        .as_str();

    let templates = captures
        .get(2)
        .expect("Static regex always has two groups")
        .as_str();

    if templates.is_empty() {
        return (
            String::new(),
            class_name_without_templates.to_string(),
            HashMap::new(),
        );
    }

    let templates: Vec<String> = split_templates(templates);
    let mut templates_string = "template <".to_string();
    let mut templates_map: HashMap<String, String> = HashMap::new();

    for (i, template) in templates.iter().enumerate() {
        let identifier = number_to_template_type_name(i);

        templates_string.push_str("typename ");
        templates_string.push_str(&identifier);
        templates_string.push(',');

        templates_map.insert(identifier, template.to_owned());
    }
    templates_string.pop(); // remove the last comma
    templates_string.push('>');
    templates_string.push(' ');

    (
        templates_string,
        class_name_without_templates.to_string(),
        templates_map,
    )
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Calling conventions for functions.
/// <https://learn.microsoft.com/en-us/visualstudio/debugger/debug-interface-access/cv-call-e?view=vs-2022>
pub enum CvCallE {
    CV_CALL_NEAR_C = 0x00,      // near right to left push, caller pops stack
    CV_CALL_FAR_C = 0x01,       // far right to left push, caller pops stack
    CV_CALL_NEAR_PASCAL = 0x02, // near left to right push, callee pops stack
    CV_CALL_FAR_PASCAL = 0x03,  // far left to right push, callee pops stack
    CV_CALL_NEAR_FAST = 0x04,   // near left to right push with regs, callee pops stack
    CV_CALL_FAR_FAST = 0x05,    // far left to right push with regs, callee pops stack
    CV_CALL_SKIPPED = 0x06,     // skipped (unused) call index
    CV_CALL_NEAR_STD = 0x07,    // near standard call
    CV_CALL_FAR_STD = 0x08,     // far standard call
    CV_CALL_NEAR_SYS = 0x09,    // near sys call
    CV_CALL_FAR_SYS = 0x0a,     // far sys call
    CV_CALL_THISCALL = 0x0b,    // this call (this passed in register)
    CV_CALL_MIPSCALL = 0x0c,    // Mips call
    CV_CALL_GENERIC = 0x0d,     // Generic call sequence
    CV_CALL_ALPHACALL = 0x0e,   // Alpha call
    CV_CALL_PPCCALL = 0x0f,     // PPC call
    CV_CALL_SHCALL = 0x10,      // Hitachi SuperH call
    CV_CALL_ARMCALL = 0x11,     // ARM call
    CV_CALL_AM33CALL = 0x12,    // AM33 call
    CV_CALL_TRICALL = 0x13,     // TriCore Call
    CV_CALL_SH5CALL = 0x14,     // Hitachi SuperH-5 call
    CV_CALL_M32RCALL = 0x15,    // M32R Call
    CV_CALL_CLRCALL = 0x16,     // clr call
    CV_CALL_INLINE = 0x17,      // Marker for routines always inlined and thus lacking a convention
    CV_CALL_NEAR_VECTOR = 0x18, // near left to right push with regs, callee pops stack
    CV_CALL_SWIFT = 0x19,       // Swift calling convention
    CV_CALL_RESERVED = 0x20,    // first unused call enumeration

                                // Do NOT add any more machine specific conventions.  This is to be used for
                                // calling conventions in the source only (e.g. __cdecl, __stdcall).
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallingConvention(u8);

impl CallingConvention {
    /// Returns the calling convention as a C++ string.
    /// <https://learn.microsoft.com/en-us/cpp/cpp/argument-passing-and-naming-conventions?view=msvc-170>
    pub fn as_cpp(self) -> &'static str {
        let cv = self.as_cvcall();
        match cv {
            CvCallE::CV_CALL_NEAR_C => "", // This is __cdecl. It's the default, so we don't need to specify it.
            CvCallE::CV_CALL_NEAR_FAST => "__fastcall ",
            CvCallE::CV_CALL_NEAR_STD => "__stdcall ",
            CvCallE::CV_CALL_NEAR_SYS => "__syscall ",
            CvCallE::CV_CALL_THISCALL => "__thiscall ",
            CvCallE::CV_CALL_CLRCALL => "__clrcall ",
            CvCallE::CV_CALL_NEAR_VECTOR => "__vectorcall ",
            _ => todo!("Unsupported calling convention: {cv:?}: {}", self.0),
        }
    }

    pub fn as_cvcall(self) -> CvCallE {
        match self.0 {
            0x00 => CvCallE::CV_CALL_NEAR_C,
            0x01 => CvCallE::CV_CALL_FAR_C,
            0x02 => CvCallE::CV_CALL_NEAR_PASCAL,
            0x03 => CvCallE::CV_CALL_FAR_PASCAL,
            0x04 => CvCallE::CV_CALL_NEAR_FAST,
            0x05 => CvCallE::CV_CALL_FAR_FAST,
            0x06 => CvCallE::CV_CALL_SKIPPED,
            0x07 => CvCallE::CV_CALL_NEAR_STD,
            0x08 => CvCallE::CV_CALL_FAR_STD,
            0x09 => CvCallE::CV_CALL_NEAR_SYS,
            0x0a => CvCallE::CV_CALL_FAR_SYS,
            0x0b => CvCallE::CV_CALL_THISCALL,
            0x0c => CvCallE::CV_CALL_MIPSCALL,
            0x0d => CvCallE::CV_CALL_GENERIC,
            0x0e => CvCallE::CV_CALL_ALPHACALL,
            0x0f => CvCallE::CV_CALL_PPCCALL,
            0x10 => CvCallE::CV_CALL_SHCALL,
            0x11 => CvCallE::CV_CALL_ARMCALL,
            0x12 => CvCallE::CV_CALL_AM33CALL,
            0x13 => CvCallE::CV_CALL_TRICALL,
            0x14 => CvCallE::CV_CALL_SH5CALL,
            0x15 => CvCallE::CV_CALL_M32RCALL,
            0x16 => CvCallE::CV_CALL_CLRCALL,
            0x17 => CvCallE::CV_CALL_INLINE,
            0x18 => CvCallE::CV_CALL_NEAR_VECTOR,
            0x19 => CvCallE::CV_CALL_SWIFT,
            0x20 => CvCallE::CV_CALL_RESERVED,
            _ => panic!("Unknown calling convention: {}", self.0),
        }
    }
}

fn typedefs() -> HashMap<&'static str, &'static str> {
    let mut typedefs: HashMap<&str, &str> = HashMap::new();
    typedefs.insert(
        "std::basic_string<char,std::char_traits<char>,std::allocator<char> >",
        "std::string",
    );
    typedefs.insert(
        "std::basic_ostringstream<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t> >",
        "std::wostringstream",
    );
    typedefs.insert(
        "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t> >",
        "std::wstring",
    );
    typedefs.insert(
        "std::basic_string<char16_t,std::char_traits<char16_t>,std::allocator<char16_t> >",
        "std::u16string",
    );
    typedefs.insert(
        "std::basic_string<char32_t,std::char_traits<char32_t>,std::allocator<char32_t> >",
        "std::u32string",
    );
    typedefs
}

/// Get all possible forward refrences at the top of the header.hpp so bindgen/clang doesn't compain.
pub fn decompile_forward_refrences(
    type_finder: &pdb::TypeFinder<'_>,
    type_information: &pdb::TypeInformation,
    lambda_names: &mut Vec<String>,
) -> String {
    let mut forward_refrences = HashSet::new();

    let _ = type_information.iter().for_each(|symbol| {
        let type_index = symbol.index();
        let Ok(data) = type_finder.find(type_index)?.parse() else {
            return Ok(());
        };

        if is_std_namespace(&data) {
            return Ok(());
        }

        match &data {
            pdb::TypeData::Class(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            pdb::TypeData::Union(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            _ => return Ok(()),
        }

        let dc = DecompilationResult::from_data(None, type_finder, data);
        let forward_refrence = dc.namespaced_repersentation();
        let forward_refrence = parse_lambdas(&forward_refrence, lambda_names);

        forward_refrences.insert(forward_refrence);

        Ok(())
    });

    let mut forward_refrences: Vec<String> = forward_refrences.into_iter().collect();
    forward_refrences.sort();
    let mut forward_refrences = forward_refrences.join("\n");
    forward_refrences.push('\n');
    forward_refrences
}

fn parse_lambdas(header_file: &str, lambda_names: &mut Vec<String>) -> String {
    let header_file = regex!(r"<lambda_(\w+?)>")
        .replace_all(header_file, |lambda_name: &regex::Captures| {
            let lambda_name = lambda_name
                .get(1)
                .expect("Compiled regex will always have a capture group")
                .as_str()
                .to_owned();
            lambda_names.push(lambda_name);
            "__#@#LAMBDA@##__"
        })
        .into_owned();
    header_file
}

fn is_std_namespace(data: &pdb::TypeData<'_>) -> bool {
    data.name().map_or(true, |name| {
        let name = name.to_string();
        name.starts_with("std::") || name.starts_with("MplVector<") || name.starts_with("Signal<")
    })
}

fn sort_with_dependencies(
    classes_and_unions: &HashMap<String, DecompilationResult>,
) -> Result<Vec<String>> {
    let mut topo_sort = TopoSort::with_capacity(classes_and_unions.len());
    for dc in classes_and_unions.values() {
        let dependencies = dc.dependencies.borrow();
        let dependencies: Vec<String> = dependencies
            .iter()
            .filter(|d| classes_and_unions.contains_key(*d))
            .cloned()
            .collect();
        //println!("{} depends on {dependencies:?}", dc.raw_name());
        topo_sort.insert(dc.raw_name().to_string(), dependencies);
    }

    match topo_sort.into_vec_nodes() {
        SortResults::Full(nodes) => Ok(nodes),
        SortResults::Partial(_) => bail!("Partial sort results"),
    }
}

fn decompile_classes_unions_and_enums(
    type_finder: &pdb::TypeFinder<'_>,
    type_information: &pdb::TypeInformation,
    lambda_names: &mut Vec<String>,
) -> String {
    // this is a hashmap becuase enums in the factorio pdb seem to exist multiple times with the same name.
    // we are taking only the most recent occurance.
    let mut enums: HashMap<String, String> = HashMap::new();
    let mut classes_and_unions: HashMap<String, DecompilationResult> = HashMap::new();

    let progressbar = indicatif::ProgressBar::new(type_information.len() as u64);
    let mut i = 0;
    let delta = 87;

    let _ = type_information.iter().for_each(|symbol| {
        i += 1;
        if i % delta == 0 {
            progressbar.inc(delta);
        }

        let type_index = symbol.index();
        let Ok(data) = type_finder.find(type_index)?.parse() else {
            return Ok(());
        };

        if is_std_namespace(&data) {
            return Ok(());
        }

        let mut is_enum = false;
        match &data {
            pdb::TypeData::Class(data)
                if !data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            pdb::TypeData::Union(data)
                if !data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            pdb::TypeData::Enumeration(data)
                if !data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() =>
            {
                is_enum = true;
            }
            _ => return Ok(()),
        };

        let dc = DecompilationResult::from_data(None, type_finder, data);

        if is_enum {
            let s = dc.namespaced_repersentation();
            let s = parse_lambdas(&s, lambda_names);
            enums.insert(dc.name, s);
        } else {
            classes_and_unions.insert(dc.raw_name().to_string(), dc);
        }

        Ok(())
    });

    let enums: HashSet<String> = enums.drain().map(|(_, v)| v).collect();
    let mut enums: Vec<String> = enums.into_iter().collect();
    enums.sort();

    let sort_order =
        sort_with_dependencies(&classes_and_unions).unwrap_or_else(|_| vec!["err!".to_string()]);
    let mut classes_and_unions: Vec<String> = sort_order
        .into_iter()
        .map(|name| {
            let dc = classes_and_unions
                .get(&name)
                .expect("sort order will have all the names");
            let s = dc.namespaced_repersentation();
            parse_lambdas(&s, lambda_names)
        })
        .collect();

    let mut classes_unions_and_enums = enums;
    classes_unions_and_enums.append(&mut classes_and_unions);
    let mut classes_unions_and_enums = classes_unions_and_enums.join("\n");
    classes_unions_and_enums.push('\n');
    classes_unions_and_enums
}

fn replace_pointers_to_errors(s: &str) -> String {
    regex!(r"/\* error processing type index 0x([\da-z]+) Support for types of kind 0x([\da-z]+) is not implemented\*/\*+;").replace_all(s,
        |captures: &regex::Captures| {
            let type_index = captures.get(1).expect("Compiled regex will always have a capture group").as_str();
            let kind = captures.get(2).expect("Compiled regex will always have a capture group").as_str();
            format!("/* error processing type index 0x{type_index} Support for types of kind 0x{kind} is not implemented*/;")
        }).into_owned()
}

fn replace_unnamed_types(s: &str) -> String {
    regex!(r"<unnamed-(type|enum)-(.+?)>")
        .replace_all(s, |captures: &regex::Captures| {
            captures
                .get(2)
                .expect("Compiled regex will always have a capture group")
                .as_str()
                .to_owned()
        })
        .into_owned()
}

pub fn generate(pdb_path: &Path) -> Result<()> {
    let pdb = File::open(pdb_path)?;
    let mut pdb = pdb::PDB::open(pdb)?;

    let type_information = pdb.type_information()?;
    let mut type_finder = type_information.finder();

    let mut type_iter = type_information.iter();
    while (type_iter.next()?).is_some() {
        // keep building the index
        type_finder.update(&type_iter);
    }

    let mut lambda_names = Vec::new();
    let mut classes_unions_and_enums =
        decompile_classes_unions_and_enums(&type_finder, &type_information, &mut lambda_names);
    let forward_refrences =
        decompile_forward_refrences(&type_finder, &type_information, &mut lambda_names);

    let mut typedefs_str = String::new();
    for (from, to) in typedefs() {
        let old_length = classes_unions_and_enums.len();
        classes_unions_and_enums = classes_unions_and_enums.replace(from, to);
        let new_length = classes_unions_and_enums.len();
        let savings = old_length - new_length;
        println!("{to} typedef saved {savings} bytes");
        typedefs_str.push_str(&format!("// typedef {from} {to};\n"));
    }

    let includes = ["<cstdint>", "<chrono>", "<vector>", "<memory>", "<string>"];
    let includes: Vec<String> = includes.iter().map(|i| format!("#include {i}")).collect();
    let includes = includes.join("\n");

    let header_file = format!("// automatically generated by pdb2hpp\n// do not edit\n\n{includes}\n\n{typedefs_str}\n\n{forward_refrences}\n{classes_unions_and_enums}");
    let mut header_file = header_file.replace("`anonymous-namespace'", "anonymous");

    let mut lambda_name_map = HashMap::new();
    header_file = regex!("__#@#LAMBDA@##__").replace_all(&header_file, |_: &regex::Captures| {
        let lambda_name = lambda_names.pop().expect("lambda_names will have the same length as the number of lambdas in the header file");
        let len = lambda_name_map.len();
        let i = lambda_name_map.entry(lambda_name).or_insert(len);
        format!("lambda_{i}")
    }).into_owned();

    header_file = replace_unnamed_types(&header_file);
    header_file = replace_pointers_to_errors(&header_file);

    println!(
        "Writing to structs.hpp. File size: {} bytes.",
        header_file.len()
    );
    let mut hpp = File::create("./src/structs.hpp")?;
    hpp.write_all(header_file.as_bytes())?;
    println!("Structs.hpp creation succeeded.");

    println!("Using bindgen to generate structs.rs");
    let bindings = bindgen::Builder::default()
        .header_contents("structs.hpp", &header_file)
        .rust_target(bindgen::RustTarget::Nightly)
        .generate()?;

    println!("Bindgen generation succeeded. Writing rust bindings to structs.rs");
    bindings.write_to_file("./src/structs.rs")?;

    Ok(())
}

fn do_indent(s: &str) -> String {
    format!("\t{}", s.replace('\n', "\n\t"))
}

fn modifier_string(modifiers: pdb::ModifierType) -> String {
    let mut s = String::new();
    if modifiers.constant {
        s.push_str("const ");
    }
    if modifiers.volatile {
        s.push_str("volatile ");
    }
    if modifiers.unaligned {
        s.push_str("__unaligned ");
    }
    s
}
