use anyhow::Result;
use core::panic;
use lazy_regex::regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use pdb::{FallibleIterator, RawString};

fn primitive_name(data: pdb::PrimitiveType) -> String {
    #[allow(clippy::match_same_arms)]
    let mut name = match data.kind {
        pdb::PrimitiveKind::Void => "void".to_string(),
        pdb::PrimitiveKind::Char => "char".to_string(),
        pdb::PrimitiveKind::UChar => "unsigned char".to_string(),
        pdb::PrimitiveKind::I8 => "int8_t".to_string(),
        pdb::PrimitiveKind::U8 => "uint8_t".to_string(),
        pdb::PrimitiveKind::I16 => "int16_t".to_string(),
        pdb::PrimitiveKind::U16 => "uint16_t".to_string(),
        pdb::PrimitiveKind::I32 => "int32_t".to_string(),
        pdb::PrimitiveKind::U32 => "uint32_t".to_string(),
        pdb::PrimitiveKind::I64 => "int64_t".to_string(),
        pdb::PrimitiveKind::U64 => "uint64_t".to_string(),
        pdb::PrimitiveKind::F32 => "float".to_string(),
        pdb::PrimitiveKind::F64 => "double".to_string(),
        pdb::PrimitiveKind::Bool8 => "bool".to_string(),

        pdb::PrimitiveKind::NoType => "void".to_string(),
        pdb::PrimitiveKind::RChar => "char".to_string(), // Assuming RChar is regular char
        pdb::PrimitiveKind::WChar => "wchar_t".to_string(),
        pdb::PrimitiveKind::RChar16 => "char16_t".to_string(),
        pdb::PrimitiveKind::RChar32 => "char32_t".to_string(),
        pdb::PrimitiveKind::Short => "short".to_string(),
        pdb::PrimitiveKind::UShort => "unsigned short".to_string(),
        pdb::PrimitiveKind::Long => "long".to_string(),
        pdb::PrimitiveKind::ULong => "unsigned long".to_string(),
        pdb::PrimitiveKind::Quad => "int64_t".to_string(), // Quad is typically 64-bit
        pdb::PrimitiveKind::UQuad => "uint64_t".to_string(),
        pdb::PrimitiveKind::Octa => "int128_t".to_string(), // Assuming Octa is 128-bit
        pdb::PrimitiveKind::UOcta => "uint128_t".to_string(),
        pdb::PrimitiveKind::I128 => "int128_t".to_string(),
        pdb::PrimitiveKind::U128 => "uint128_t".to_string(),
        pdb::PrimitiveKind::F16 => "__fp16".to_string(), // Assuming F16 is half-precision floating-point
        pdb::PrimitiveKind::F32PP => "float _Complex".to_string(), // Assuming F32PP is complex float
        pdb::PrimitiveKind::F48 => "float _Complex".to_string(), // Assuming F48 is complex float (needs confirmation)
        pdb::PrimitiveKind::F80 => "long double".to_string(),
        pdb::PrimitiveKind::F128 => "__float128".to_string(), // Quad-precision floating-point
        pdb::PrimitiveKind::Complex32 => "_Complex float".to_string(),
        pdb::PrimitiveKind::Complex64 => "_Complex double".to_string(),
        pdb::PrimitiveKind::Complex80 => "_Complex long double".to_string(),
        pdb::PrimitiveKind::Complex128 => "_Complex __float128".to_string(), // Assuming complex quad-precision
        pdb::PrimitiveKind::Bool16 => "_Bool16".to_string(), // Assuming 16-bit boolean
        pdb::PrimitiveKind::Bool32 => "_Bool32".to_string(), // Assuming 32-bit boolean
        pdb::PrimitiveKind::Bool64 => "_Bool64".to_string(), // Assuming 64-bit boolean
        pdb::PrimitiveKind::HRESULT => "HRESULT".to_string(),
        _ => panic!("Unknown primitive type {:?}", data.kind),
    };

    if data.indirection.is_some() {
        name.push('*');
    }

    name
}

struct FieldAttributes(pdb::FieldAttributes, bool);

impl FieldAttributes {
    fn as_string(&self) -> String {
        let mut s = String::new();
        if self.0.is_static() {
            s = format!("static {s}");
        }
        if self.1 || self.0.is_virtual() || self.0.is_pure_virtual() {
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
    if properties.constructors() {
        s.push_str("constructors ");
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

#[allow(clippy::too_many_lines)]
fn convert_pdb_data_to_cpp_code(
    type_finder: &pdb::TypeFinder<'_>,
    type_data: pdb::TypeData<'_>,
    base_classes: &mut Vec<String>,
) -> Result<String, pdb::Error> {
    Ok(match type_data {
        pdb::TypeData::Member(data) => {
            // A field inside a class
            Field {
                type_name: type_index_to_string(type_finder, data.field_type),
                name: data.name.to_string().into(),
                offset: Some(data.offset),
            }
            .as_string()
        }
        pdb::TypeData::StaticMember(data) => {
            let type_data = type_finder.find(data.field_type)?.parse()?;
            let s = convert_pdb_data_to_cpp_code(type_finder, type_data, base_classes)?;
            format!("static {s}")
        }

        pdb::TypeData::BaseClass(data) => {
            let attributes = FieldAttributes(data.attributes, false).as_string();
            let type_name = type_index_to_string(type_finder, data.base_class);

            base_classes.push(format!("{attributes}{type_name}"));

            format!(
                "/* offset {:3} */ /* fields for {} */",
                data.offset, type_name
            )
        }

        pdb::TypeData::Primitive(data) => primitive_name(data), // A primitive type in c++. See primitive_name() for a full list.
        pdb::TypeData::Class(data) => {
            // A class type in c++
            // TODO handle data.properties
            Class::new(data).as_string(type_finder)
        }
        pdb::TypeData::FieldList(data) => {
            // A list of fields inside a class
            let mut fields: Vec<String> = Vec::new();
            for field in data.fields {
                let err = format!("/* field type not found: {field:?} */");
                let is_enumerate = matches!(field, pdb::TypeData::Enumerate(_));
                let mut s = convert_pdb_data_to_cpp_code(type_finder, field.clone(), base_classes)
                    .unwrap_or(err);

                if !is_enumerate {
                    // if we dont have an enum, we need to add a semicolon
                    s.push(';');
                }
                fields.push(s);
            }
            if let Some(continuation) = data.continuation {
                // recurse
                fields.push(convert_pdb_data_to_cpp_code_from_index(
                    type_finder,
                    continuation,
                    base_classes,
                )?);
            }
            fields.join("\n")
        }
        pdb::TypeData::Union(data) => {
            // A union type in c++
            // TODO handle data.properties
            let type_name = data.name.to_string().into_owned();
            let (templates, type_name, templates_map) = parse_template_types(&type_name);

            if data.properties.forward_reference() {
                return Ok(format!("union {type_name}"));
            }

            let fields =
                convert_pdb_data_to_cpp_code_from_index(type_finder, data.fields, base_classes)?;
            let mut fields = do_indent(&fields);

            for (identifier, template) in templates_map {
                fields = fields.replace(&template, &identifier);
            }

            format!(
                "{}{templates}union {type_name} {{\n{fields}\n}}",
                stringify_properties(data.properties),
            )
        }
        pdb::TypeData::Enumerate(data) => {
            // One of the values inside an enum
            format!("{} = {},", data.name, data.value)
        }
        pdb::TypeData::Enumeration(data) => {
            // An enum type in c++
            // TODO handle data.properties
            let type_name = data.name.to_string().into_owned();

            if data.properties.forward_reference() {
                return Ok(format!("enum {type_name}"));
            }

            let fields =
                convert_pdb_data_to_cpp_code_from_index(type_finder, data.fields, base_classes)?;
            let fields = do_indent(&fields);

            format!(
                "{}enum {type_name} /* stored as {} */ {{\n{fields}\n}}",
                stringify_properties(data.properties),
                data.underlying_type,
            )
        }
        pdb::TypeData::Pointer(data) => {
            // Pointer to a diffrent datatype
            let mut s = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.underlying_type,
                base_classes,
            )?;
            s.push('*');
            s
        }
        pdb::TypeData::Modifier(data) => {
            // Wrapper around another type that describes a modifier. Can be const, volatile, or unaligned.
            let s = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.underlying_type,
                base_classes,
            )?;
            format!("{}{s}", modifier_string(data))
        }
        pdb::TypeData::Array(data) => {
            let mut suffix: String = String::new();
            for size in data.dimensions {
                suffix = format!("{suffix}[{size}]");
            }
            let name = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.element_type,
                base_classes,
            )?;
            format!("{name}{suffix}")
        }
        pdb::TypeData::Procedure(data) => {
            let args = argument_list(type_finder, data.argument_list)?.join(", ");
            let return_type = data.return_type.map_or_else(
                || "void".to_string(),
                |return_type| type_index_to_string(type_finder, return_type),
            );
            format!("{return_type}({args})")
        }
        pdb::TypeData::Nested(data) => {
            let s = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.nested_type,
                base_classes,
            )?;
            format!("{}{s}", FieldAttributes(data.attributes, false).as_string())
        }
        pdb::TypeData::Method(data) => {
            Method::new(type_finder, data.name, data.attributes, data.method_type)?.as_string()
        }

        pdb::TypeData::OverloadedMethod(data) => {
            // this just means we have more than one method with the same name
            // find the method list
            match type_finder.find(data.method_list)?.parse()? {
                pdb::TypeData::MethodList(method_list) => {
                    let mut s = Vec::new();
                    for pdb::MethodListEntry {
                        attributes,
                        method_type,
                        ..
                    } in method_list.methods
                    {
                        // hooray
                        let method = Method::new(type_finder, data.name, attributes, method_type)?;

                        s.push(method.as_string());
                    }
                    s.join("\n")
                }
                other => {
                    format!("processing OverloadedMethod, expected MethodList, got {} -> {:?}. unexpected type in Class::add_field()", data.method_list, other)
                }
            }
        }
        pdb::TypeData::MemberFunction(data) => Method {
            name: RawString::from(""), // todo!
            return_type_name: type_index_to_string(type_finder, data.return_type),
            arguments: argument_list(type_finder, data.argument_list)?,
            attributes: data.attributes,
            is_virtual: false,
            is_static: false,
        }
        .as_string(),
        pdb::TypeData::VirtualBaseClass(data) => format!("todo! VirtualBaseClass {data:?}"),
        pdb::TypeData::VirtualFunctionTablePointer(data) => {
            let s = convert_pdb_data_to_cpp_code_from_index(type_finder, data.table, base_classes)?;
            format!("{s}*")
        }
        pdb::TypeData::Bitfield(data) => {
            let s = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.underlying_type,
                base_classes,
            )?;
            format!("{} : {}", s, data.length)
        }
        pdb::TypeData::ArgumentList(data) => format!("todo! ArgumentList {data:?}"),
        pdb::TypeData::MethodList(data) => format!("todo! MethodList {data:?}"),
        _ => todo!(),
    })
}

fn convert_pdb_data_to_cpp_code_from_index(
    type_finder: &pdb::TypeFinder<'_>,
    type_index: pdb::TypeIndex,
    base_classes: &mut Vec<String>,
) -> Result<String, pdb::Error> {
    let data = type_finder.find(type_index)?.parse()?;
    convert_pdb_data_to_cpp_code(type_finder, data, base_classes)
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
            '<' => {
                template_depth += 1;
                current.push('<');
            }
            '>' => {
                template_depth -= 1;
                current.push('>');
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

    #[allow(clippy::expect_used)]
    let class_name_without_templates = captures
        .get(1)
        .expect("Static regex always has one group")
        .as_str();

    #[allow(clippy::expect_used)]
    let templates = captures
        .get(2)
        .expect("Static regex always has two groups")
        .as_str();

    if templates.is_empty() {
        return (String::new(), class_name.to_string(), HashMap::new());
    }

    let templates: Vec<String> = split_templates(templates);
    let mut templates_string = "template <".to_string();
    let mut templates_map: HashMap<String, String> = HashMap::new();

    for (i, template) in templates.iter().enumerate() {
        let identifier = number_to_template_type_name(i);

        templates_string.push_str("typename ");
        templates_string.push_str(&identifier);
        templates_string.push(',');
        templates_string.push(' ');

        templates_map.insert(identifier, template.to_owned());
    }
    templates_string.pop(); // remove the last space
    templates_string.pop(); // remove the last comma
    templates_string.push('>');
    templates_string.push(' ');

    (
        templates_string,
        class_name_without_templates.to_string(),
        templates_map,
    )
}

struct Class<'p> {
    data: pdb::ClassType<'p>,
    base_classes: Vec<String>,
}

impl<'p> Class<'p> {
    const fn new(data: pdb::ClassType<'p>) -> Self {
        Class {
            data,
            base_classes: Vec::new(),
        }
    }

    fn as_string(&mut self, type_finder: &'p pdb::TypeFinder) -> String {
        let name = self.data.name.to_string().into_owned();
        let (templates, name, templates_map) = parse_template_types(&name);
        let kind = match self.data.kind {
            pdb::ClassKind::Class => "class",
            pdb::ClassKind::Struct => "struct",
            pdb::ClassKind::Interface => "interface", // when can this happen?
        };

        if self.data.properties.forward_reference() {
            return format!("{templates}{kind} {name}");
        }

        let mut fields: Vec<String> = Vec::new();

        if let Some(derived_from) = self.data.derived_from {
            if let Err(e) = self.add_derived_from(type_finder, derived_from) {
                fields.push(format!("/* derived from error: {e} */"));
            }
        }

        if let Some(type_index) = self.data.fields {
            match convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                type_index,
                &mut self.base_classes,
            ) {
                Ok(f) => fields.push(f),
                Err(e) => {
                    fields.push(format!("/* add field error: {e} */"));
                }
            }
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
            stringify_properties(self.data.properties)
        )
    }

    #[allow(clippy::unnecessary_wraps)]
    fn add_derived_from(
        &self,
        _: &pdb::TypeFinder<'p>,
        _: pdb::TypeIndex,
    ) -> Result<(), pdb::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    type_name: String,
    name: String,
    offset: Option<u64>,
}

impl Field {
    fn as_string(&self) -> String {
        self.offset.map_or_else(
            || format!("{} {}", self.type_name, self.name),
            |offset| format!("/* offset {offset:3} */ {} {}", self.type_name, self.name),
        )
    }
}

fn type_index_to_string(type_finder: &pdb::TypeFinder<'_>, type_index: pdb::TypeIndex) -> String {
    match type_finder.find(type_index) {
        Ok(t) => match t.parse() {
            Ok(pdb::TypeData::Pointer(data)) => format!(
                "{}*",
                type_index_to_string(type_finder, data.underlying_type)
            ),

            Ok(pdb::TypeData::Primitive(data)) => primitive_name(data),

            Ok(data) if matches!(data, pdb::TypeData::Procedure(_)) => {
                convert_pdb_data_to_cpp_code(type_finder, data, &mut Vec::new()).unwrap_or_else(
                    |e| format!("/* error processing type index {type_index} {e}*/"),
                )
            }

            Ok(pdb::TypeData::Modifier(data)) => {
                format!(
                    "{}{}",
                    modifier_string(data),
                    type_index_to_string(type_finder, data.underlying_type)
                )
            }

            Ok(data) => data.name().map_or_else(
                || {
                    convert_pdb_data_to_cpp_code(type_finder, data, &mut Vec::new()).unwrap_or_else(
                        |e| format!("/* error processing type index {type_index} {e}*/"),
                    )
                },
                |type_name| type_name.to_string().into_owned(),
            ),

            Err(e) => format!("/* error parsing type index {type_index} {e}*/"),
        },
        Err(e) => format!("/* error finding type index {type_index} {e}*/"),
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Method<'p> {
    name: pdb::RawString<'p>,
    return_type_name: String,
    arguments: Vec<String>,
    attributes: pdb::FunctionAttributes,
    is_virtual: bool,
    is_static: bool,
}

impl<'p> Method<'p> {
    fn new(
        type_finder: &pdb::TypeFinder<'p>,
        name: pdb::RawString<'p>,
        attributes: pdb::FieldAttributes,
        type_index: pdb::TypeIndex,
    ) -> Result<Self, pdb::Error> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::MemberFunction(data) => {
                let method = Method {
                    name,
                    return_type_name: type_index_to_string(type_finder, data.return_type),
                    arguments: argument_list(type_finder, data.argument_list)?,
                    attributes: data.attributes,
                    is_virtual: attributes.is_virtual(),
                    is_static: attributes.is_static(),
                };
                Ok(method)
            }

            other => {
                panic!(
                    "processing Method, expected MemberFunction, got {} -> {:?}. unexpected type in Method::find()",
                    type_index, other
                )
            }
        }
    }

    fn as_string(&self) -> String {
        let return_type = if self.attributes.is_constructor() {
            ""
        } else {
            &format!("{} ", self.return_type_name)
        };

        let name = self.name.to_string();
        let arguments = self.arguments.join(", ");

        let calling_convention = CallingConvention(self.attributes.calling_convention()).as_cpp();

        format!(
            "{}{}{return_type}{calling_convention}{name}({arguments})",
            if self.is_static { "static " } else { "" },
            if self.is_virtual { "virtual " } else { "" },
        )
    }
}

fn argument_list(
    type_finder: &pdb::TypeFinder<'_>,
    type_index: pdb::TypeIndex,
) -> Result<Vec<String>, pdb::Error> {
    match type_finder.find(type_index)?.parse()? {
        pdb::TypeData::ArgumentList(data) => {
            let mut args: Vec<String> = Vec::new();
            for arg_type in data.arguments {
                args.push(type_index_to_string(type_finder, arg_type));
            }
            Ok(args)
        }
        _ => todo!("argument list of non-argument-list type"),
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
) -> String {
    let mut forward_refrences = HashSet::new();

    let _ = type_information.iter().for_each(|symbol| {
        let type_index = symbol.index();
        let Ok(data) = type_finder.find(type_index)?.parse() else {
            return Ok(());
        };

        match &data {
            pdb::TypeData::Class(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            pdb::TypeData::Union(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            pdb::TypeData::Enumeration(data)
                if data.properties.forward_reference()
                    && !data.properties.is_nested_type()
                    && !data.properties.scoped_definition() => {}
            _ => return Ok(()),
        }

        let name = data.name().unwrap().to_string();
        let forward_refrence = convert_pdb_data_to_cpp_code(type_finder, data, &mut Vec::new());
        let forward_refrence = forward_refrence
            .unwrap_or_else(|e| format!("/* error processing type index {type_index} {e}*/"));
        let forward_refrence = parse_namespaces(&forward_refrence, &name);

        forward_refrences.insert(forward_refrence);

        Ok(())
    });

    let mut forward_refrences: Vec<String> = forward_refrences.into_iter().collect();
    forward_refrences.sort();
    let mut forward_refrences = forward_refrences.join("\n");
    forward_refrences.push('\n');
    forward_refrences
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

    let includes = ["<cstdint>"];
    let includes: Vec<String> = includes.iter().map(|i| format!("#include {i}")).collect();
    let includes = includes.join("\n");

    let progressbar = indicatif::ProgressBar::new(type_information.len() as u64);
    let mut i = 0;
    let delta = 87;

    let mut classes_unions_and_enums: HashSet<String> = HashSet::new();
    let _ = type_information.iter().for_each(|symbol| {
        if i % delta == 0 {
            progressbar.inc(delta);
        }
        i += 1;

        let type_index = symbol.index();
        let data = match type_finder.find(type_index)?.parse() {
            Ok(data) => data,
            Err(e) => {
                classes_unions_and_enums
                    .insert(format!("/* error processing type index {symbol:?} {e}*/\n"));
                return Ok(());
            }
        };

        match &data {
            // The type_information list contains all types, not just top-level types. We need to filter for only classes, unions, enums.
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
                    && !data.properties.scoped_definition() => {}
            _ => return Ok(()),
        }

        let name = data.name().unwrap().to_string();
        let s = convert_pdb_data_to_cpp_code(&type_finder, data, &mut Vec::new());
        let s = s.unwrap_or_else(|e| format!("/* error processing type index {type_index} {e}*/"));
        let s = parse_namespaces(&s, &name);

        classes_unions_and_enums.insert(s);
        Ok(())
    });

    let mut classes_unions_and_enums: Vec<String> = classes_unions_and_enums.into_iter().collect();
    classes_unions_and_enums.sort();
    let mut classes_unions_and_enums = classes_unions_and_enums.join("\n");
    classes_unions_and_enums.push('\n');

    let forward_refrences = decompile_forward_refrences(&type_finder, &type_information);

    let mut typedefs_str = String::new();
    for (from, to) in typedefs() {
        let old_length = classes_unions_and_enums.len();
        classes_unions_and_enums = classes_unions_and_enums.replace(from, to);
        let new_length = classes_unions_and_enums.len();
        let savings = old_length - new_length;
        println!("{to} typedef saved {savings} bytes");
        typedefs_str.push_str(&format!("// typedef {from} {to};\n"));
    }

    let header_file = format!("// automatically generated by pdb2hpp\n// do not edit\n\n{includes}\n\n{typedefs_str}\n\n{forward_refrences}\n{classes_unions_and_enums}");
    let header_file = header_file.replace("`anonymous-namespace'", "anonymous");

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
