use anyhow::Result;
use core::panic;
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
        name.push_str(" *");
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

#[allow(clippy::too_many_lines)]
fn convert_pdb_data_to_cpp_code(
    type_finder: &pdb::TypeFinder<'_>,
    type_data: pdb::TypeData<'_>,
    base_classes: &mut Vec<String>,
) -> Result<String, pdb::Error> {
    Ok(match type_data {
        pdb::TypeData::Member(data) => Field {
            // A field inside a class
            type_name: convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.field_type,
                base_classes,
            )?,
            name: data.name.to_string().into(),
            offset: Some(data.offset),
        }
        .as_string(),

        pdb::TypeData::StaticMember(data) => Field {
            // A static field inside a class
            type_name: format!(
                "static {}",
                convert_pdb_data_to_cpp_code_from_index(
                    type_finder,
                    data.field_type,
                    base_classes
                )?
            ),
            name: data.name.to_string().into(),
            offset: None,
        }
        .as_string(),

        pdb::TypeData::BaseClass(data) => {
            let attributes = FieldAttributes(data.attributes, false).as_string();
            let type_name = type_data.name().map_or_else(
                || "<unknown parent class>".to_string(),
                |type_name| type_name.to_string().into_owned(),
            );

            base_classes.push(format!("{attributes}{type_name}"));

            format!(
                "/* offset {:3} */ /* fields for {} */\n",
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
                fields.push(
                    convert_pdb_data_to_cpp_code(type_finder, field, base_classes).unwrap_or(err),
                );
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

            let fields =
                convert_pdb_data_to_cpp_code_from_index(type_finder, data.fields, base_classes)?;
            let fields = do_indent(&fields);

            format!("union {type_name} {{\n{fields}\n}};")
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
                "enum {type_name} /* stored as {} */ {{\n{fields}\n}};",
                data.underlying_type,
            )
        }
        pdb::TypeData::Pointer(data) => { // Pointer to a diffrent datatype
            let mut s = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.underlying_type,
                base_classes,
            )?;
            if s.ends_with(';') {
                s.pop();
                s.push('*');
                s.push(';');
            } else {
                s.push('*');
            }
            s
        }
        pdb::TypeData::Modifier(data) => {
            // Wrapper around another type that describes a modifier. Can be const, volatile, or unaligned.
            let mut s = convert_pdb_data_to_cpp_code_from_index(
                type_finder,
                data.underlying_type,
                base_classes,
            )?;
            if data.constant {
                s = format!("const {s}");
            }
            if data.volatile {
                s = format!("volatile {s}");
            }
            if data.unaligned {
                s = format!("__unaligned {s}");
            }
            s
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
            if let Some(return_type) = data.return_type {
                let return_type = convert_pdb_data_to_cpp_code_from_index(
                    type_finder,
                    return_type,
                    base_classes,
                )?;
                let args = argument_list(type_finder, data.argument_list)?;
                format!("{return_type}({})", args.join(", "))
            } else {
                let args = argument_list(type_finder, data.argument_list)?;
                format!("void({})", args.join(", "))
            }
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
            Method::find(type_finder, data.name, data.attributes, data.method_type)?.as_string()
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
                        let method = Method::find(type_finder, data.name, attributes, method_type)?;

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
            return format!("{name};");
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
        format!("{name} {{\n{fields}\n}};")
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
            || format!("{} {};", self.type_name, self.name),
            |offset| format!("/* offset {offset:3} */ {} {};", self.type_name, self.name),
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

impl<'p> Method<'p> {
    fn find(
        type_finder: &pdb::TypeFinder<'p>,
        name: pdb::RawString<'p>,
        attributes: pdb::FieldAttributes,
        type_index: pdb::TypeIndex,
    ) -> Result<Self, pdb::Error> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::MemberFunction(data) => Ok(Method {
                name,
                return_type_name: convert_pdb_data_to_cpp_code_from_index(
                    type_finder,
                    data.return_type,
                    &mut Vec::new(),
                )?,
                arguments: argument_list(type_finder, data.argument_list)?,
                is_virtual: attributes.is_virtual(),
                is_static: attributes.is_static(),
            }),

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
            "{}{}{} {}({});",
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
                args.push(convert_pdb_data_to_cpp_code_from_index(
                    type_finder,
                    arg_type,
                    &mut Vec::new(),
                )?);
            }
            Ok(args)
        }
        _ => todo!("argument list of non-argument-list type"),
    }
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
    let mut header: String =
        "// automatically generated by pdb2hpp\n// do not edit\n\n".to_string();
    header.push_str(&includes.join("\n"));
    header.push('\n');
    header.push('\n');

    let mut i = 0;
    let _ = type_information.iter().for_each(|symbol| {
        let type_index = symbol.index();
        let data = match type_finder.find(type_index)?.parse() {
            Ok(data) => data,
            Err(e) => {
                println!("Error parsing type index {symbol:?} {e}");
                return Ok(());
            }
        };

        i += 1;
        if i % 1000 == 0 {
            println!("Processing type index {i}");
        }

        if let pdb::TypeData::FieldList(_) = data {
            // I'm not sure why field lists show up in the top level PDB. Skip them.
            return Ok(());
        }

        header.push_str(
            &convert_pdb_data_to_cpp_code(&type_finder, data, &mut Vec::new())
                .unwrap_or_else(|e| format!("/* error processing type index {type_index} {e}*/")),
        );
        header.push('\n');
        Ok(())
    });

    println!("Writing to structs.hpp. File size: {} bytes.", header.len());

    let mut hpp = File::create("./src/structs.hpp")?;
    hpp.write_all(header.as_bytes())?;

    println!("Using bindgen to generate structs.rs");

    let bindings = bindgen::Builder::default()
        .header_contents("structs.hpp", &header)
        .rust_target(bindgen::RustTarget::Nightly)
        .generate()?;

    println!("Writing to structs.rs");

    bindings.write_to_file("./src/structs.rs")?;

    Ok(())
}

fn do_indent(s: &str) -> String {
    format!("\t{}", s.replace('\n', "\n\t"))
}
