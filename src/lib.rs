use pdb::{FallibleIterator, PDB};
use retour::static_detour;
use std::error::Error;
use std::ffi::c_char;
use std::iter;
use std::net::TcpStream;
use std::sync::Mutex;
use std::{collections::HashMap, ffi::c_int, fs::File, mem};
use tracing::{error, info};
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
    base_address: u64,
}

impl PDBCache {
    fn new(pdb_path: &str, module_name: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;
        let base_address = unsafe { get_dll_base_address(module_name)? };

        let mut cache = Self {
            pdb,
            symbol_addresses: HashMap::new(),
            base_address,
        };

        cache.build_symbol_map()?;
        //cache.print_structs()?;

        Ok(cache)
    }

    fn build_symbol_map(&mut self) -> Result<(), Box<dyn Error>> {
        let symbol_table = self.pdb.global_symbols()?;
        let address_map = self.pdb.address_map()?;

        symbol_table.iter().for_each(|symbol| {
            match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                    self.symbol_addresses
                        .insert(data.name.to_string().into(), rva.0);
                }
                Err(e) => return Err(e),
                _ => {}
            };

            Ok(())
        })?;

        Ok(())
    }

    pub fn get_function_address(&self, function_name: &str) -> Option<u64> {
        self.symbol_addresses
            .get(function_name)
            .copied()
            .map(|x| self.base_address + u64::from(x))
    }

    pub fn print_structs(&mut self) -> Result<(), Box<dyn Error>> {
        let symbol_table = self.pdb.type_information()?;
        info!("Symbol table:");
        symbol_table.iter().for_each(|symbol| {
            match symbol.parse() {
                Ok(pdb::TypeData::Primitive(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Class(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Member(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::MemberFunction(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::OverloadedMethod(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Method(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::StaticMember(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Nested(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::BaseClass(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::VirtualBaseClass(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::VirtualFunctionTablePointer(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Procedure(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Pointer(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Modifier(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Enumeration(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Enumerate(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Array(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Union(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::Bitfield(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::FieldList(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::ArgumentList(data)) => {
                    info!("{:?}", data)
                }
                Ok(pdb::TypeData::MethodList(data)) => {
                    info!("{:?}", data)
                }
                Err(e) => return Err(e),
                _ => {}
            };

            Ok(())
        })?;

        Ok(())
    }
}

unsafe fn get_dll_base_address(module_name: &str) -> Result<u64, Box<dyn Error>> {
    let result = GetModuleHandleA(module_name.as_pcstr());
    match result {
        Ok(handle) => Ok(handle.0 as u64),
        Err(err) => Err(err.into()),
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

trait AsPcwstr {
    fn as_pcwstr(&self) -> PCWSTR;
}

impl AsPcwstr for str {
    fn as_pcwstr(&self) -> PCWSTR {
        let utf16: Vec<u16> = self.encode_utf16().chain(iter::once(0)).collect();
        PCWSTR(utf16.as_ptr().cast())
    }
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

unsafe fn hook(pdb_cache: &PDBCache) -> Result<(), Box<dyn Error>> {
    let function_name = "?valid@LuaSurface@@UEBA_NXZ";

    let Some(address) = pdb_cache.get_function_address(function_name) else {
        return Err("Failed to find main address".into());
    };

    info!("{} address: {:#x}", function_name, address);
    let target: FnMain = mem::transmute(address);
    MainHook.initialize(target, main_detour)?.enable()?;
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    let ip = "127.0.0.1:40267";
    #[allow(clippy::expect_used, clippy::expect_fun_call)]
    let stream = TcpStream::connect(ip).expect(&format!(
        "Could not establish stdout connection to rivets. Port {ip} is busy."
    ));
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
