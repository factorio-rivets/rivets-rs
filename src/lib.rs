use anyhow::{bail, Result};
use pdb::{FallibleIterator, TypeIndex, PDB};
use retour::static_detour;
use std::ffi::c_char;
use std::iter;
use std::net::TcpStream;
use std::sync::Mutex;
use std::{collections::HashMap, ffi::c_int, fs::File, mem};
use tracing::{error, info, warn};
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
    base_address: u64,
}

impl PDBCache {
    fn new(pdb_path: &str, module_name: &str) -> Result<Self> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;
        let base_address = unsafe { get_dll_base_address(module_name)? };

        let mut cache = Self {
            pdb,
            symbol_addresses: HashMap::new(),
            base_address,
        };

        cache.build_symbol_map()?;
        cache.print_structs()?;

        Ok(cache)
    }

    fn build_symbol_map(&mut self) -> Result<()> {
        let symbol_table = self.pdb.global_symbols()?;
        let address_map = self.pdb.address_map()?;

        symbol_table
            .iter()
            .for_each(|symbol| match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                    self.symbol_addresses
                        .insert(data.name.to_string().into(), rva.0);
                    Ok(())
                }
                Err(e) => Err(e),
                _ => Ok(()),
            })?;

        Ok(())
    }

    pub fn get_function_address(&self, function_name: &str) -> Option<u64> {
        self.symbol_addresses
            .get(function_name)
            .copied()
            .map(|x| self.base_address + u64::from(x))
    }

    pub fn print_structs(&mut self) -> Result<()> {
        /*
            2024-07-11T05:24:15.380140Z  INFO examplemod: Class count: 124843
            2024-07-11T05:24:15.380303Z  INFO examplemod: Member function count: 1
            2024-07-11T05:24:15.380362Z  INFO examplemod: Procedure count: 65550
            2024-07-11T05:24:15.380413Z  INFO examplemod: Pointer count: 177468
            2024-07-11T05:24:15.380457Z  INFO examplemod: Modifier count: 39642
            2024-07-11T05:24:15.380523Z  INFO examplemod: Enumeration count: 4752
            2024-07-11T05:24:15.380602Z  INFO examplemod: Array count: 1731
            2024-07-11T05:24:15.380673Z  INFO examplemod: Union count: 1010
            2024-07-11T05:24:15.380715Z  INFO examplemod: Bitfield count: 216
            2024-07-11T05:24:15.380758Z  INFO examplemod: Field list count: 57236
            2024-07-11T05:24:15.380802Z  INFO examplemod: Argument list count: 193345
            2024-07-11T05:24:15.380844Z  INFO examplemod: Method list count: 86779
         */
        
        let symbol_table = self.pdb.type_information()?;
        info!("Symbol table:");
        let mut class_count: u64 = 0;
        let mut member_function_count: u64 = 0;
        let mut procedure_count: u64 = 0;
        let mut pointer_count: u64 = 0;
        let mut modifier_count: u64 = 0;
        let mut enumeration_count: u64 = 0;
        let mut array_count: u64 = 0;
        let mut union_count: u64 = 0;
        let mut bitfield_count: u64 = 0;
        let mut field_list_count: u64 = 0;
        let mut argument_list_count: u64 = 0;
        let mut method_list_count: u64 = 0;

        let mut finder: pdb::ItemFinder<TypeIndex> = symbol_table.finder();
        let mut type_iter = type_information.iter();
            while let Some(typ) = type_information.item()? {
                // keep building the index
                type_finder.update(&type_iter);
            }

        symbol_table.iter().for_each(|symbol| {
            match symbol.parse() {
                Ok(pdb::TypeData::Class(data)) => {
                    if class_count % 1000 == 0 {
                        info!("{}", data.name);
                        let Some(field_index) = data.fields else {
                            warn!("No fields found for class {}", data.name);
                            return Ok(())
                        };
                        let fields: pdb::Item<TypeIndex> = finder.find(field_index)?;
                        info!("q");
                        let field_data: pdb::TypeData = fields.parse()?; // Failed to parse Factorio PDB file. Type 4152 not indexed (index covers 4103)
                        info!("{field_data:?}");
                    }
                    class_count += 1;
                },
                Ok(pdb::TypeData::MemberFunction(_)) => {
                    member_function_count= 1;
                },
                Ok(pdb::TypeData::Procedure(_)) => {
                    procedure_count += 1;
                },
                Ok(pdb::TypeData::Pointer(_)) => {
                    pointer_count += 1;
                },
                Ok(pdb::TypeData::Modifier(_)) => {
                    modifier_count += 1;
                },
                Ok(pdb::TypeData::Enumeration(_)) => {
                    enumeration_count += 1;
                },
                Ok(pdb::TypeData::Array(_)) => {
                    array_count += 1;
                },
                Ok(pdb::TypeData::Union(_)) => {
                    union_count += 1;
                },
                Ok(pdb::TypeData::Bitfield(_)) => {
                    bitfield_count += 1;
                },
                Ok(pdb::TypeData::FieldList(_)) => {
                    field_list_count += 1;
                },
                Ok(pdb::TypeData::ArgumentList(_)) => {
                    argument_list_count += 1;
                },
                Ok(pdb::TypeData::MethodList(_)) => {
                    method_list_count += 1;
                },
                _ => {}
            };
            Ok(())
        })?;

        let justify: usize = 16;
        info!("Class count: {class_count:>justify$}");
        info!("Member function count: {member_function_count:>justify$}");
        info!("Procedure count: {procedure_count:>justify$}");
        info!("Pointer count: {pointer_count:>justify$}");
        info!("Modifier count: {modifier_count:>justify$}");
        info!("Enumeration count: {enumeration_count:>justify$}");
        info!("Array count: {array_count:>justify$}");
        info!("Union count: {union_count:>justify$}");
        info!("Bitfield count: {bitfield_count:>justify$}");
        info!("Field list count: {field_list_count:>justify$}");
        info!("Argument list count: {argument_list_count:>justify$}");
        info!("Method list count: {method_list_count:>justify$}");
        

        Ok(())
    }
}

unsafe fn get_dll_base_address(module_name: &str) -> Result<u64> {
    let result = GetModuleHandleA(module_name.as_pcstr());
    match result {
        Ok(handle) => Ok(handle.0 as u64),
        Err(err) => bail!(err),
    }
}

static_detour! {
    static MainHook: unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> bool;
}

type FnMain = unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> bool;

fn main_detour(_argc: c_int, _argv: *const c_char, _envp: *const c_char) -> bool {
    info!("Detoured into main!");
    //unsafe { MessageBoxWHook.call(hwnd, text, replaced_caption, msgbox_style) }
    false
}

trait AsPcstr {
    unsafe fn as_pcstr(&self) -> PCSTR;
}

impl AsPcstr for str {
    /// This is unsafe because `PCSTR` does not implement the `Sized` trait and thus its bytes may be overwritten.
    /// Use the `s!` macro to create a safe `PCSTR` at compile-time.
    unsafe fn as_pcstr(&self) -> PCSTR {
        PCSTR(format!("{self}\0").as_ptr().cast())
    }
}

unsafe fn hook(pdb_cache: &PDBCache) -> Result<()> {
    let function_name = "?valid@LuaSurface@@UEBA_NXZ";

    let Some(address) = pdb_cache.get_function_address(function_name) else {
        bail!("Failed to find main address");
    };

    info!("{} address: {:#x}", function_name, address);
    let target: FnMain = mem::transmute(address);
    MainHook.initialize(target, main_detour)?.enable()?;
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    let ip = "127.0.0.1:40267";
    let stream = TcpStream::connect(ip).unwrap_or_else(|_| {
        panic!("Could not establish stdout connection to rivets. Port {ip} is busy.")
    });
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    let cache = match PDBCache::new(
        r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb",
        "factorio.exe",
    ) {
        Ok(cache) => cache,
        Err(e) => {
            error!("Failed to parse Factorio PDB file. {e:?}");
            return;
        }
    };

    let result = unsafe { hook(&cache) };
    if let Err(e) = result {
        error!("{e:?}");
    }
}
