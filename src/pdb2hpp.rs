use anyhow::{bail, Result};
use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::{collections::BTreeSet, path::Path};
use std::fmt;

use pdb::{FallibleIterator, ItemInformation, TypeIndex};

type TypeSet = BTreeSet<pdb::TypeIndex>;

pub fn variable_identifier_suffix(
    type_finder: &pdb::TypeFinder<'_>,
    type_index: pdb::TypeIndex,
) -> Result<String> {
    let suffix = match type_finder.find(type_index)?.parse()? {
        pdb::TypeData::Array(data) => {
            let mut suffix: String = String::new();
            for size in data.dimensions {
                suffix = format!("{suffix}[{size}]");
            }
            suffix
        }

        _ => String::new(),
    };

    Ok(suffix)
}

pub fn type_name(
    type_finder: &pdb::TypeFinder<'_>,
    type_index: pdb::TypeIndex,
    needed_types: &mut TypeSet,
) -> Result<String> {
    let mut name = match type_finder.find(type_index)?.parse()? {
        pdb::TypeData::Primitive(data) => {
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

        pdb::TypeData::Class(data) => {
            needed_types.insert(type_index);
            data.name.to_string().into_owned()
        }

        pdb::TypeData::Enumeration(data) => {
            needed_types.insert(type_index);
            data.name.to_string().into_owned()
        }

        pdb::TypeData::Union(data) => {
            needed_types.insert(type_index);
            data.name.to_string().into_owned()
        }

        pdb::TypeData::Pointer(data) => format!(
            "{}*",
            type_name(type_finder, data.underlying_type, needed_types)?
        ),

        pdb::TypeData::Modifier(data) => {
            if data.constant {
                format!(
                    "const {}",
                    type_name(type_finder, data.underlying_type, needed_types)?
                )
            } else if data.volatile {
                format!(
                    "volatile {}",
                    type_name(type_finder, data.underlying_type, needed_types)?
                )
            } else {
                // ?
                type_name(type_finder, data.underlying_type, needed_types)?
            }
        }

        pdb::TypeData::Array(data) => type_name(type_finder, data.element_type, needed_types)?,

        _ => format!("Type{type_index} /* TODO: figure out how to name it */"),
    };

    // TODO: search and replace std:: patterns
    if name == "std::basic_string<char,std::char_traits<char>,std::allocator<char> >" {
        name = "std::string".to_string();
    }

    Ok(name)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Class<'p> {
    kind: pdb::ClassKind,
    name: pdb::RawString<'p>,
    base_classes: Vec<BaseClass>,
    fields: Vec<Field>,
    instance_methods: Vec<Method<'p>>,
    static_methods: Vec<Method<'p>>,
}

impl<'p> Class<'p> {
    #[allow(clippy::unnecessary_wraps)]
    fn add_derived_from(
        &self,
        _: &pdb::TypeFinder<'p>,
        _: pdb::TypeIndex,
        _: &mut TypeSet,
    ) -> Result<()> {
        todo!()
    }

    fn add_fields(
        &mut self,
        type_finder: &pdb::TypeFinder<'p>,
        type_index: pdb::TypeIndex,
        needed_types: &mut TypeSet,
    ) -> Result<()> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::FieldList(data) => {
                for field in &data.fields {
                    self.add_field(type_finder, field, needed_types)?;
                }

                if let Some(continuation) = data.continuation {
                    // recurse
                    self.add_fields(type_finder, continuation, needed_types)?;
                }
            }
            other => {
                println!("trying to Class::add_fields() got {type_index} -> {other:?}");
                panic!("unexpected type in Class::add_fields()");
            }
        }

        Ok(())
    }

    fn add_field(
        &mut self,
        type_finder: &pdb::TypeFinder<'p>,
        field: &pdb::TypeData<'p>,
        needed_types: &mut TypeSet,
    ) -> Result<()> {
        match *field {
            pdb::TypeData::Member(ref data) => {
                // TODO: attributes (static, virtual, etc.)
                self.fields.push(Field {
                    type_name: type_name(type_finder, data.field_type, needed_types)?,
                    name: data.name.to_string().into(),
                    offset: data.offset,
                    suffix: variable_identifier_suffix(type_finder, data.field_type)?,
                });
            }

            pdb::TypeData::Method(ref data) => {
                let method = Method::find(
                    data.name,
                    data.attributes,
                    type_finder,
                    data.method_type,
                    needed_types,
                )?;
                if data.attributes.is_static() {
                    self.static_methods.push(method);
                } else {
                    self.instance_methods.push(method);
                }
            }

            pdb::TypeData::OverloadedMethod(ref data) => {
                // this just means we have more than one method with the same name
                // find the method list
                match type_finder.find(data.method_list)?.parse()? {
                    pdb::TypeData::MethodList(method_list) => {
                        for pdb::MethodListEntry {
                            attributes,
                            method_type,
                            ..
                        } in method_list.methods
                        {
                            // hooray
                            let method = Method::find(
                                data.name,
                                attributes,
                                type_finder,
                                method_type,
                                needed_types,
                            )?;

                            if attributes.is_static() {
                                self.static_methods.push(method);
                            } else {
                                self.instance_methods.push(method);
                            }
                        }
                    }
                    other => {
                        println!(
                            "processing OverloadedMethod, expected MethodList, got {} -> {:?}",
                            data.method_list, other
                        );
                        panic!("unexpected type in Class::add_field()");
                    }
                }
            }

            pdb::TypeData::BaseClass(ref data) => self.base_classes.push(BaseClass {
                type_name: type_name(type_finder, data.base_class, needed_types)?,
                offset: data.offset,
            }),

            pdb::TypeData::VirtualBaseClass(ref data) => self.base_classes.push(BaseClass {
                type_name: type_name(type_finder, data.base_class, needed_types)?,
                offset: data.base_pointer_offset,
            }),

            _ => {
                // ignore everything else even though that's sad
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BaseClass {
    type_name: String,
    offset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    type_name: String,
    name: String,
    offset: u64,
    suffix: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Method<'p> {
    name: pdb::RawString<'p>,
    return_type_name: String,
    arguments: Vec<String>,
    is_virtual: bool,
}

impl<'p> Method<'p> {
    fn find(
        name: pdb::RawString<'p>,
        attributes: pdb::FieldAttributes,
        type_finder: &pdb::TypeFinder<'p>,
        type_index: pdb::TypeIndex,
        needed_types: &mut TypeSet,
    ) -> Result<Self> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::MemberFunction(data) => Ok(Method {
                name,
                return_type_name: type_name(type_finder, data.return_type, needed_types)?,
                arguments: argument_list(type_finder, data.argument_list, needed_types)?,
                is_virtual: attributes.is_virtual(),
            }),

            other => {
                println!("other: {other:?}");
                todo!("that")
            }
        }
    }
}

fn argument_list(
    type_finder: &pdb::TypeFinder<'_>,
    type_index: pdb::TypeIndex,
    needed_types: &mut TypeSet,
) -> Result<Vec<String>> {
    match type_finder.find(type_index)?.parse()? {
        pdb::TypeData::ArgumentList(data) => {
            let mut args: Vec<String> = Vec::new();
            for arg_type in data.arguments {
                args.push(type_name(type_finder, arg_type, needed_types)?);
            }
            Ok(args)
        }
        _ => todo!("argument list of non-argument-list type"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Union<'p> {
    name: pdb::RawString<'p>,
    fields: Vec<Field>,
}

impl<'p> Union<'p> {
    fn add_fields(
        &mut self,
        type_finder: &pdb::TypeFinder<'p>,
        type_index: pdb::TypeIndex,
        needed_types: &mut TypeSet,
    ) -> Result<()> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::FieldList(data) => {
                for field in &data.fields {
                    self.add_field(type_finder, field, needed_types)?;
                }

                if let Some(continuation) = data.continuation {
                    // recurse
                    self.add_fields(type_finder, continuation, needed_types)?;
                }
            }
            pdb::TypeData::Primitive(data) => {
                self.fields.push(Field {
                    type_name: type_name(type_finder, type_index, needed_types)?,
                    name: format!("{:?}", data.kind),
                    offset: 0,
                    suffix: String::new(),
                });
            }
            other => {
                println!("trying to Union::add_fields() got {type_index} -> {other:?}");
                panic!("unexpected type in Union::add_fields()");
            }
        }

        Ok(())
    }
    fn add_field(
        &mut self,
        type_finder: &pdb::TypeFinder<'p>,
        field: &pdb::TypeData<'p>,
        needed_types: &mut TypeSet,
    ) -> Result<()> {
        // ignore everything else even though that's sad
        if let pdb::TypeData::Member(ref data) = field {
            self.fields.push(Field {
                type_name: type_name(type_finder, data.field_type, needed_types)?,
                name: data.name.to_string().into(),
                offset: 0,
                suffix: variable_identifier_suffix(type_finder, data.field_type)?,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Enum<'p> {
    name: pdb::RawString<'p>,
    underlying_type_name: String,
    values: Vec<EnumValue<'p>>,
}

impl<'p> Enum<'p> {
    fn add_fields(
        &mut self,
        type_finder: &pdb::TypeFinder<'p>,
        type_index: pdb::TypeIndex,
        needed_types: &mut TypeSet,
    ) -> Result<()> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::FieldList(data) => {
                for field in &data.fields {
                    self.add_field(type_finder, field, needed_types);
                }

                if let Some(continuation) = data.continuation {
                    // recurse
                    self.add_fields(type_finder, continuation, needed_types)?;
                }
            }
            other => {
                println!("trying to Enum::add_fields() got {type_index} -> {other:?}");
                panic!("unexpected type in Enum::add_fields()");
            }
        }

        Ok(())
    }

    fn add_field(&mut self, _: &pdb::TypeFinder<'p>, field: &pdb::TypeData<'p>, _: &mut TypeSet) {
        // ignore everything else even though that's sad
        if let pdb::TypeData::Enumerate(ref data) = field {
            self.values.push(EnumValue {
                name: data.name,
                value: data.value,
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EnumValue<'p> {
    name: pdb::RawString<'p>,
    value: pdb::Variant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForwardReference<'p> {
    kind: pdb::ClassKind,
    name: pdb::RawString<'p>,
}

#[derive(Debug, Clone)]
struct Data<'p> {
    forward_references: Vec<ForwardReference<'p>>,
    classes: HashSet<Class<'p>>,
    enums: HashSet<Enum<'p>>,
    unions: HashSet<Union<'p>>,
}

impl fmt::Display for Data<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.forward_references.is_empty() {
            writeln!(f)?;
            for e in &self.forward_references {
                e.fmt(f)?;
            }
        }

        for union in &self.unions {
            writeln!(f)?;
            union.fmt(f)?;
        }

        for enu in &self.enums {
            writeln!(f)?;
            enu.fmt(f)?;
        }

        for class in &self.classes {
            writeln!(f)?;
            class.fmt(f)?;
        }

        Ok(())
    }
}

impl<'p> Data<'p> {
    fn new() -> Self {
        Data {
            forward_references: Vec::new(),
            classes: HashSet::new(),
            enums: HashSet::new(),
            unions: HashSet::new(),
        }
    }

    fn add(
        &mut self,
        type_finder: &pdb::TypeFinder<'p>,
        type_index: pdb::TypeIndex,
        needed_types: &mut TypeSet,
    ) -> Result<()> {
        match type_finder.find(type_index)?.parse()? {
            pdb::TypeData::Class(data) => {
                if data.properties.forward_reference() {
                    self.forward_references.push(ForwardReference {
                        kind: data.kind,
                        name: data.name,
                    });

                    return Ok(());
                }

                let mut class = Class {
                    kind: data.kind,
                    name: data.name,
                    fields: Vec::new(),
                    base_classes: Vec::new(),
                    instance_methods: Vec::new(),
                    static_methods: Vec::new(),
                };

                if let Some(derived_from) = data.derived_from {
                    class.add_derived_from(type_finder, derived_from, needed_types)?;
                }

                if let Some(fields) = data.fields {
                    class.add_fields(type_finder, fields, needed_types)?;
                }

                self.classes.replace(class);
            }

            pdb::TypeData::Enumeration(data) => {
                let mut e = Enum {
                    name: data.name,
                    underlying_type_name: type_name(
                        type_finder,
                        data.underlying_type,
                        needed_types,
                    )?,
                    values: Vec::new(),
                };

                e.add_fields(type_finder, data.fields, needed_types)?;

                self.enums.replace(e);
            }

            pdb::TypeData::Union(data) => {
                let mut union = Union {
                    name: data.name,
                    fields: Vec::new(),
                };

                union.add_fields(type_finder, data.fields, needed_types)?;

                self.unions.replace(union);
            }

            // ignore
            other => eprintln!("warning: don't know how to add {other:?}"),
        }

        Ok(())
    }
}

fn write_class<'a, 'b> (
    type_information: &ItemInformation<TypeIndex>,
    type_finder: &pdb::TypeFinder<'a>,
    needed_types: &mut TypeSet,
    data: &mut Data<'b>,
    class_name: &str,
) -> Result<String> where 'a: 'b {
    let mut type_iter = type_information.iter();
    //let current_size = data.classes.len() + data.enums.len() + data.unions.len();

    while let Some(typ) = type_iter.next()? {
        match typ.parse() {
            Ok(pdb::TypeData::Class(class)) => {
                if class.name.as_bytes() == class_name.as_bytes()
                    && !class.properties.forward_reference()
                {
                    data.add(type_finder, typ.index(), needed_types)?;
                    break;
                }
            }
            Ok(pdb::TypeData::Union(union)) => {
                if union.name.as_bytes() == class_name.as_bytes() {
                    data.add(type_finder, typ.index(), needed_types)?;
                    break;
                }
            }
            _ => {}
        }
    }

    /*let new_size = data.classes.len() + data.enums.len() + data.unions.len();
    if new_size == current_size {
        bail!("sorry, class {} was not found", class_name);
    }*/

    Ok(format!("{data}"))
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
    let mut header: String = "// automatically generated by pdb2hpp\n// do not edit\n\n".to_string();
    header.push_str(&includes.join("\n"));

    let needed_class_names = vec![
        "lua_State",
        "lua_Debug",
        "global_State",
        "stringtable",
        "Mbuffer",
        "lua_longjmp",
        //"CallInfoInternalC",
        //"CallInfoInternalL",
        "CallInfo",
        //"CallInfoInternal",
        "Upvaldesc",
        "LocVar",
        //"UpValInternalInternal",
        //"UpValInternal",
        "UpVal",
        "Proto",
        "GCObject",
        "Closure",
        "LClosure",
        "CClosure",
        "Node",
        //"TKeyInner",
        "TKey",
        "Value",
        //"TValue",
        "Table",
        //"UdataInner",
        "Udata",
        //"TStringInner",
        "TString",
        "L_Umaxalign",
        "GCheader",
    ];

    let mut needed_types = TypeSet::new();
    let mut data: Data = Data::new();

    for class_name in needed_class_names {
        header.push_str(&write_class(
            &type_information,
            &type_finder,
            &mut needed_types,
            &mut data,
            class_name,
        )?);
    }

    while let Some(type_index) = needed_types.iter().next_back().copied() {
        // remove it
        needed_types.remove(&type_index);

        // add the type
        data.add(type_finder, type_index, needed_types)?;
    }

    println!("{header}");

    let bindings = bindgen::Builder::default()
        .header_contents("bindings.hpp", &header)
        .rust_target(bindgen::RustTarget::Nightly)
        .generate()?;

    bindings.write_to_file("./src/structs.rs")?;

    Ok(())
}

//////////////////
// impl Display //
//////////////////

impl fmt::Display for Class<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ",
            match self.kind {
                pdb::ClassKind::Class => "class",
                pdb::ClassKind::Struct => "struct",
                pdb::ClassKind::Interface => "interface", // when can this happen?
            },
            self.name.to_string()
        )?;

        if !self.base_classes.is_empty() {
            for (i, base) in self.base_classes.iter().enumerate() {
                let prefix = match i {
                    0 => ":",
                    _ => ",",
                };
                write!(f, "{} {}", prefix, base.type_name)?;
            }
        }

        writeln!(f, " {{")?;

        for base in &self.base_classes {
            writeln!(
                f,
                "\t/* offset {:3} */ /* fields for {} */",
                base.offset, base.type_name
            )?;
        }

        for field in &self.fields {
            writeln!(
                f,
                "\t/* offset {:3} */ {} {}{};",
                field.offset,
                field.type_name,
                field.name,
                field.suffix,
            )?;
        }

        if !self.instance_methods.is_empty() {
            writeln!(f, "\t")?;
            for method in &self.instance_methods {
                writeln!(
                    f,
                    "\t{}{} {}({});",
                    if method.is_virtual { "virtual " } else { "" },
                    method.return_type_name,
                    method.name.to_string(),
                    method.arguments.join(", ")
                )?;
            }
        }

        if !self.static_methods.is_empty() {
            writeln!(f, "\t")?;
            for method in &self.static_methods {
                writeln!(
                    f,
                    "\t{}static {} {}({});",
                    if method.is_virtual { "virtual " } else { "" },
                    method.return_type_name,
                    method.name.to_string(),
                    method.arguments.join(", ")
                )?;
            }
        }

        writeln!(f, "}};")?;

        Ok(())
    }
}

impl fmt::Display for Union<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "union {} {{", self.name.to_string())?;

        for field in &self.fields {
            writeln!(
                f,
                "\t{} {}{};",
                field.type_name,
                field.name,
                field.suffix
            )?;
        }

        writeln!(f, "}};")?;

        Ok(())
    }
}

impl fmt::Display for Enum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "enum {} /* stored as {} */ {{",
            self.name.to_string(),
            self.underlying_type_name
        )?;

        for value in &self.values {
            writeln!(
                f,
                "\t{} = {},",
                value.name.to_string(),
                match value.value {
                    pdb::Variant::U8(v) => format!("0x{v:02x}"),
                    pdb::Variant::U16(v) => format!("0x{v:04x}"),
                    pdb::Variant::U32(v) => format!("0x{v:08x}"),
                    pdb::Variant::U64(v) => format!("0x{v:16x}"),
                    pdb::Variant::I8(v) => format!("{v}"),
                    pdb::Variant::I16(v) => format!("{v}"),
                    pdb::Variant::I32(v) => format!("{v}"),
                    pdb::Variant::I64(v) => format!("{v}"),
                }
            )?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl fmt::Display for ForwardReference<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {};",
            match self.kind {
                pdb::ClassKind::Class => "class",
                pdb::ClassKind::Struct => "struct",
                pdb::ClassKind::Interface => "interface", // when can this happen?
            },
            self.name.to_string()
        )
    }
}

impl<'p> Hash for Class<'p> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'p> Hash for Enum<'p> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'p> Hash for Union<'p> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}