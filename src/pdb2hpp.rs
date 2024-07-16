use anyhow::Result;
use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use pdb::FallibleIterator;

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

            if data.properties.forward_reference() {
                return Ok(format!("union {type_name}"));
            }

            let fields =
                convert_pdb_data_to_cpp_code_from_index(type_finder, data.fields, base_classes)?;
            let fields = do_indent(&fields);

            format!(
                "{}union {type_name} {{\n{fields}\n}}",
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
        pdb::TypeData::MemberFunction(data) => format!("todo! MemberFunction {data:?}"),
        pdb::TypeData::VirtualBaseClass(data) => format!("todo! VirtualBaseClass {data:?}"),
        pdb::TypeData::VirtualFunctionTablePointer(data) => {
            format!("todo! VirtualFunctionTablePointer {data:?}")
        }
        pdb::TypeData::Bitfield(data) => format!("todo! Bitfield {data:?}"),
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
        let mut name = format!(
            "{} {}",
            match self.data.kind {
                pdb::ClassKind::Class => "class",
                pdb::ClassKind::Struct => "struct",
                pdb::ClassKind::Interface => "interface", // when can this happen?
            },
            self.data.name.to_string()
        );

        if self.data.properties.forward_reference() {
            return name.to_string();
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

        if !self.base_classes.is_empty() {
            for (i, base) in self.base_classes.iter().enumerate() {
                let prefix = if i == 0 { ':' } else { ',' };
                name.push_str(&format!("{prefix} {base}"));
            }
        }

        let fields = do_indent(&fields.join("\n"));
        format!("{}{name} {{\n{fields}\n}}", stringify_properties(self.data.properties))
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Method<'p> {
    name: pdb::RawString<'p>,
    return_type_name: String,
    arguments: Vec<String>,
    is_virtual: bool,
    is_static: bool,
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
        format!(
            "{}{}{} {}({})",
            if self.is_static { "static " } else { "" },
            if self.is_virtual { "virtual " } else { "" },
            self.return_type_name,
            self.name.to_string(),
            self.arguments.join(", ")
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
    let delta = 47;

    let mut forward_refrences = String::new();
    let _ = type_information.iter().for_each(|symbol| {
        let type_index = symbol.index();
        let Ok(data) = type_finder.find(type_index)?.parse() else {
            return Ok(());
        };

        match &data {
            // Get all the forward refrences at the top so bindgen doesn't compain.
            pdb::TypeData::Class(data) if data.properties.forward_reference() => {}
            pdb::TypeData::Union(data) if data.properties.forward_reference() => {}
            pdb::TypeData::Enumeration(data) if data.properties.forward_reference() => {}
            _ => return Ok(()),
        }

        forward_refrences.push_str(
            &convert_pdb_data_to_cpp_code(&type_finder, data, &mut Vec::new())
                .unwrap_or_else(|e| format!("/* error processing type index {type_index} {e}*/")),
        );
        forward_refrences.push(';');
        forward_refrences.push('\n');

        Ok(())
    });

    let mut header = String::new();
    let _ = type_information.iter().for_each(|symbol| {
        if i % delta == 0 {
            progressbar.inc(delta);
        }
        i += 1;

        let type_index = symbol.index();
        let data = match type_finder.find(type_index)?.parse() {
            Ok(data) => data,
            Err(e) => {
                header.push_str(&format!(
                    "/* error processing type index {symbol:?} {e}*/\n"
                ));
                return Ok(());
            }
        };

        match &data {
            // The type_information list contains all types, not just top-level types. We need to filter for only classes, unions, enums.
            pdb::TypeData::Class(data) if !data.properties.forward_reference() => {}
            pdb::TypeData::Union(data) if !data.properties.forward_reference() => {}
            pdb::TypeData::Enumeration(data) if !data.properties.forward_reference() => {}
            _ => return Ok(()),
        }

        header.push_str(
            &convert_pdb_data_to_cpp_code(&type_finder, data, &mut Vec::new())
                .unwrap_or_else(|e| format!("/* error processing type index {type_index} {e}*/")),
        );
        header.push(';');
        header.push('\n');
        Ok(())
    });

    let mut typedefs_str = String::new();
    for (from, to) in typedefs() {
        let old_length = header.len();
        header = header.replace(from, to);
        let new_length = header.len();
        let savings = old_length - new_length;
        println!("{to} typedef saved {savings} bytes");
        typedefs_str.push_str(&format!("// typedef {from} {to};\n"));
    }
    header = format!("// automatically generated by pdb2hpp\n// do not edit\n\n{includes}\n\n{forward_refrences}\n\n{typedefs_str}\n{header}");

    println!("Writing to structs.hpp. File size: {} bytes.", header.len());
    let mut hpp = File::create("./src/structs.hpp")?;
    hpp.write_all(header.as_bytes())?;
    println!("Structs.hpp creation succeeded.");

    println!("Using bindgen to generate structs.rs");
    let bindings = bindgen::Builder::default()
        .header_contents("structs.hpp", &header)
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
