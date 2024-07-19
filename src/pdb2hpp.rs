#![allow(clippy::expect_used)]
#![allow(clippy::trivial_regex)]

use anyhow::{bail, Result};
use core::panic;
use lazy_regex::regex;
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

struct FieldAttributes {
    attributes: pdb::FieldAttributes,
    is_virtual: bool,
}

impl FieldAttributes {
    fn as_string(&self) -> String {
        let mut s = match self.attributes.access() {
            1 => "private ",
            2 => "protected ",
            3 => "public ",
            _ => "",
        }
        .to_string();

        if self.attributes.is_static() {
            s = format!("static {s}");
        }
        if self.is_virtual || self.attributes.is_virtual() || self.attributes.is_pure_virtual() {
            s = format!("virtual {s}");
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

fn split_cpp_class_name(class_name: &str) -> Vec<String> {
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

fn parse_namespaces(data_structure: &str, name: &str) -> String {
    // this function executes after the template types have been calculated. remove everything after "<"
    let name = name.split('<').next().unwrap_or_default();

    let mut namespaces = split_cpp_class_name(name);
    let short_name = namespaces.pop().unwrap_or_default();
    if namespaces.is_empty() {
        return format!("{data_structure};");
    }

    let data_structure = data_structure.replacen(name, &short_name, 1);

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

fn find_type<'a>(
    type_finder: &pdb::TypeFinder<'a>,
    type_index: pdb::TypeIndex,
) -> Result<pdb::TypeData<'a>> {
    Ok(type_finder.find(type_index)?.parse()?)
}

struct DecompilationResult<'a> {
    repersentation: Option<String>,
    name: Option<String>,
    dependancies: HashSet<String>,
    data: Option<pdb::TypeData<'a>>,
    type_finder: &'a pdb::TypeFinder<'a>,
    base_classes: Vec<String>,
}

impl<'a> DecompilationResult<'a> {
    fn from_data(type_finder: &'a pdb::TypeFinder<'a>, data: pdb::TypeData<'a>) -> Self {
        DecompilationResult {
            repersentation: None,
            name: None,
            dependancies: HashSet::new(),
            data: Some(data),
            type_finder,
            base_classes: Vec::new(),
        }
    }

    fn from_index(type_finder: &'a pdb::TypeFinder<'a>, type_index: pdb::TypeIndex) -> Self {
        match find_type(type_finder, type_index) {
            Ok(data) => DecompilationResult::from_data(type_finder, data),
            Err(e) => DecompilationResult {
                repersentation: Some(format!("/* error processing type index {type_index} {e}*/")),
                name: Some(format!("/* error processing type index {type_index} {e}*/")),
                dependancies: HashSet::new(),
                data: None,
                type_finder,
                base_classes: Vec::new(),
            },
        }
    }

    fn name(&mut self) -> &str {
        if let Some(ref name) = self.name {
            return name;
        }

        let type_finder = self.type_finder;

        self.name = Some(match &self.data {
            Some(pdb::TypeData::Pointer(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.underlying_type);
                format!("{}*", dc.name())
            }

            Some(pdb::TypeData::Modifier(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.underlying_type);
                format!("{}{}", modifier_string(*data), dc.name())
            }

            Some(data) => match data.name() {
                Some(name) => name.to_string().into_owned(),
                None => return self.repersentation(),
            },

            _ => return self.repersentation(),
        });

        self.name()
    }

    #[allow(clippy::too_many_lines)]
    fn repersentation(&mut self) -> &str {
        if let Some(ref repersentation) = self.repersentation {
            return repersentation;
        }

        let type_finder = self.type_finder;

        self.repersentation = Some(match &mut self.data {
            Some(pdb::TypeData::Member(data)) => {
                // A field inside a class
                let mut dc = DecompilationResult::from_index(type_finder, data.field_type);
                let offset = data.offset;
                let field_type = dc.name();
                let field_name = data.name.to_string();
                format!("/* offset {offset:3} */ {field_type} {field_name}")
            }
            Some(pdb::TypeData::StaticMember(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.field_type);
                format!("static {}", dc.repersentation())
            }
            Some(pdb::TypeData::BaseClass(data)) => {
                let attributes = FieldAttributes {
                    attributes: data.attributes,
                    is_virtual: false,
                }
                .as_string();
                let mut dc = DecompilationResult::from_index(type_finder, data.base_class);
                let type_name = dc.name();

                self.base_classes.push(format!("{attributes}{type_name}"));

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
                for field in data.fields.clone() {
                    let is_enumerate = matches!(field, pdb::TypeData::Enumerate(_));
                    let mut dc = DecompilationResult::from_data(type_finder, field);
                    let mut s = dc.repersentation().to_string();

                    if !is_enumerate {
                        // if we dont have an enum, we need to add a semicolon
                        s.push(';');
                    }
                    fields.push(s);
                }
                if let Some(continuation) = data.continuation {
                    // recurse
                    let mut dc = DecompilationResult::from_index(type_finder, continuation);
                    fields.push(format!("{};", dc.repersentation()));
                }
                fields.join("\n")
            }
            Some(pdb::TypeData::Union(data)) => {
                // A union type in c++
                // TODO handle data.properties
                let type_name = data.name.to_string().into_owned();
                let (templates, type_name, templates_map) = parse_template_types(&type_name);

                if data.properties.forward_reference() {
                    self.repersentation = Some(format!("union {type_name}"));
                    return self.repersentation();
                }

                let mut dc = DecompilationResult::from_index(type_finder, data.fields);
                let mut fields = do_indent(dc.repersentation());

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
                let type_name = data.name.to_string().into_owned();

                if data.properties.forward_reference() {
                    self.repersentation = Some(format!("enum {type_name}"));
                    return self.repersentation();
                }

                let mut dc = DecompilationResult::from_index(type_finder, data.fields);
                let fields = do_indent(dc.repersentation());
                let mut underlying_dc =
                    DecompilationResult::from_index(type_finder, data.underlying_type);
                let underlying_type = underlying_dc.name();

                format!(
                    "{}enum class {type_name} : {underlying_type} {{\n{fields}\n}}",
                    stringify_properties(data.properties),
                )
            }
            Some(pdb::TypeData::Pointer(data)) => {
                // Pointer to a diffrent datatype
                let mut dc = DecompilationResult::from_index(type_finder, data.underlying_type);
                format!("{}*", dc.repersentation())
            }
            Some(pdb::TypeData::Modifier(data)) => {
                // Wrapper around another type that describes a modifier. Can be const, volatile, or unaligned.
                let mut dc = DecompilationResult::from_index(type_finder, data.underlying_type);
                format!("{}{}", modifier_string(*data), dc.repersentation())
            }
            Some(pdb::TypeData::Array(data)) => {
                let mut suffix: String = String::new();
                for size in &data.dimensions {
                    suffix = format!("{suffix}[{size}]");
                }
                let mut dc = DecompilationResult::from_index(type_finder, data.element_type);
                format!("{}{suffix}", dc.name())
            }
            Some(pdb::TypeData::Procedure(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.argument_list);
                let args = dc.repersentation();

                let return_type = data.return_type.map_or_else(
                    || "void".to_string(),
                    |return_type| {
                        let mut dc = DecompilationResult::from_index(type_finder, return_type);
                        dc.name().to_string()
                    },
                );
                format!("{return_type}({args})")
            }
            Some(pdb::TypeData::Nested(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.nested_type);
                format!(
                    "{}{}",
                    FieldAttributes {
                        attributes: data.attributes,
                        is_virtual: false
                    }
                    .as_string(),
                    dc.repersentation()
                )
            }
            Some(pdb::TypeData::Method(data)) => match find_type(type_finder, data.method_type) {
                Ok(pdb::TypeData::MemberFunction(member_function_data)) => method_to_string(
                    type_finder,
                    &data.name.to_string(),
                    member_function_data.return_type,
                    member_function_data.argument_list,
                    member_function_data.attributes,
                    Some(data.attributes),
                ),
                other => {
                    format!("processing Method, expected MethodType, got {other:?}.")
                }
            },
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
                            match find_type(type_finder, method_type) {
                                Ok(pdb::TypeData::MemberFunction(member_function_data)) => {
                                    s.push(method_to_string(
                                        type_finder,
                                        &data.name.to_string(),
                                        member_function_data.return_type,
                                        member_function_data.argument_list,
                                        member_function_data.attributes,
                                        Some(attributes),
                                    ));
                                }
                                other => {
                                    s.push(format!(
                                        "processing OverloadedMethod, expected MethodList, got {other:?}."
                                    ));
                                }
                            }
                        }
                        s.join(";\n")
                    }
                    other => {
                        format!("processing OverloadedMethod, expected MethodList, got {other:?}.")
                    }
                }
            }
            Some(pdb::TypeData::MemberFunction(data)) => method_to_string(
                type_finder,
                "",
                data.return_type,
                data.argument_list,
                data.attributes,
                None,
            ),
            Some(pdb::TypeData::VirtualBaseClass(data)) => {
                format!("todo! VirtualBaseClass {data:?}")
            }
            Some(pdb::TypeData::VirtualFunctionTablePointer(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.table);
                format!("{}*", dc.repersentation())
            }
            Some(pdb::TypeData::Bitfield(data)) => {
                let mut dc = DecompilationResult::from_index(type_finder, data.underlying_type);
                format!("{} : {}", dc.repersentation(), data.length)
            }
            Some(pdb::TypeData::ArgumentList(data)) => {
                let mut args = Vec::new();
                for arg_type in data.arguments.clone() {
                    let mut dc = DecompilationResult::from_index(type_finder, arg_type);
                    args.push(dc.name().to_string());
                }
                args.join(",")
            }
            Some(pdb::TypeData::MethodList(data)) => format!("todo! MethodList {data:?}"),
            _ => todo!(),
        });

        self.repersentation()
    }

    fn class_to_string(&mut self) -> String {
        let name = self.name().to_string();
        let data = &mut self.data;

        let Some(pdb::TypeData::Class(data)) = data else {
            return format!("/* error processing class {name} */");
        };

        let (templates, name, templates_map) = parse_template_types(&name);
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
            let mut dc = DecompilationResult::from_index(self.type_finder, type_index);
            fields.push(dc.repersentation().to_string());
            self.base_classes.extend(dc.base_classes);
            self.dependancies.extend(dc.dependancies);
        }

        let mut base_classes = String::new();
        if !self.base_classes.is_empty() {
            for (i, base) in self.base_classes.iter().enumerate() {
                let prefix = if i == 0 { ':' } else { ',' };
                base_classes.push_str(&format!("{prefix} {base}"));
            }
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
}

fn method_to_string(
    type_finder: &pdb::TypeFinder<'_>,
    method_name: &str,
    return_type: pdb::TypeIndex,
    arguments_list: pdb::TypeIndex,
    function_attributes: pdb::FunctionAttributes,
    field_attributes: Option<pdb::FieldAttributes>,
) -> String {
    let is_destructor = method_name.starts_with('~');

    let return_type = if is_destructor || function_attributes.is_constructor() {
        ""
    } else {
        let mut dc = DecompilationResult::from_index(type_finder, return_type);
        &format!("{} ", dc.name())
    };

    let calling_convention = CallingConvention(function_attributes.calling_convention()).as_cpp();
    let field_attributes = field_attributes.map_or_else(String::new, |field_attributes| {
        FieldAttributes {
            attributes: field_attributes,
            is_virtual: false,
        }
        .as_string()
    });

    let mut dc = DecompilationResult::from_index(type_finder, arguments_list);
    let arguments = dc.repersentation();

    format!("{field_attributes}{return_type}{calling_convention}{method_name}({arguments})",)
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
                result.push(current.clone());
                current.clear();
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

        let name = match &data {
            pdb::TypeData::Class(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() =>
            {
                data.name.to_string()
            }
            pdb::TypeData::Union(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() =>
            {
                data.name.to_string()
            }
            _ => return Ok(()),
        };

        let mut dc = DecompilationResult::from_data(type_finder, data);
        let forward_refrence = dc.repersentation();
        let forward_refrence = parse_namespaces(forward_refrence, &name);
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

fn sort_with_dependancies(
    mut classes_and_unions: HashMap<String, HashSet<String>>,
) -> Result<Vec<String>> {
    let mut topo_sort = TopoSort::with_capacity(classes_and_unions.len());
    for (class, unions) in classes_and_unions.drain() {
        topo_sort.insert(class, unions);
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
    let mut classes_and_unions: HashSet<String> = HashSet::new();

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
        let name = match &data {
            pdb::TypeData::Class(data)
                if !data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() =>
            {
                data.name.to_string()
            }
            pdb::TypeData::Union(data)
                if !data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() =>
            {
                data.name.to_string()
            }
            pdb::TypeData::Enumeration(data)
                if !data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() =>
            {
                is_enum = true;
                data.name.to_string()
            }
            _ => return Ok(()),
        };

        let mut dc = DecompilationResult::from_data(type_finder, data);
        let s = dc.repersentation();
        let s = parse_namespaces(s, &name);
        let s = parse_lambdas(&s, lambda_names);

        if is_enum {
            enums.insert(name.into_owned(), s);
        } else {
            classes_and_unions.insert(s);
        }

        Ok(())
    });

    let enums: HashSet<String> = enums.drain().map(|(_, v)| v).collect();
    let mut enums: Vec<String> = enums.into_iter().collect();
    enums.sort();

    let mut classes_and_unions: Vec<String> = classes_and_unions.into_iter().collect();
    classes_and_unions.sort();

    let mut classes_unions_and_enums = enums;
    classes_unions_and_enums.append(&mut classes_and_unions);
    let mut classes_unions_and_enums = classes_unions_and_enums.join("\n");
    classes_unions_and_enums.push('\n');
    classes_unions_and_enums
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

    let includes = ["<cstdint>", "<chrono>", "<vector>", "<memory>"];
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

    header_file = regex!("<unnamed-(type|enum)-(.+?)>")
        .replace_all(&header_file, |captures: &regex::Captures| {
            captures
                .get(2)
                .expect("Compiled regex will always have a capture group")
                .as_str()
                .to_owned()
        })
        .into_owned();

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
