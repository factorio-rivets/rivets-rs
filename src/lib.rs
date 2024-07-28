use anyhow::{bail, Result};
use pdb::{FallibleIterator, PDB};
use retour::static_detour;
use windows::Win32::System::Threading::CREATE_SUSPENDED;
use std::ffi::{c_char, CString};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;
use std::{collections::HashMap, ffi::c_int, fs::File, mem};
use tracing::{error, info};
use traits::{factorio_path, AsPcstr};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use std::io;
use std::net::TcpListener;
use windows::core::{s, w, PCSTR, PSTR};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, PROCESS_INFORMATION, STARTUPINFOA,
};

mod traits;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
    base_address: u64,
}

impl PDBCache {
    fn new(pdb_path: &Path, module_name: &str) -> Result<Self> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;
        let base_address = unsafe { get_dll_base_address(module_name)? };

        let mut cache = Self {
            pdb,
            symbol_addresses: HashMap::new(),
            base_address,
        };

        cache.build_symbol_map()?;

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
}

unsafe fn get_dll_base_address(module_name: &str) -> Result<u64> {
    let result = GetModuleHandleA(CString::new(module_name)?.as_pcstr());
    match result {
        Ok(handle) => Ok(handle.0 as u64),
        Err(err) => bail!(err),
    }
}

static_detour! {
    static MainHook: unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> bool;
}

type FnMain = unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> bool;

fn start_factorio(factorio_path: PCSTR) -> Result<PROCESS_INFORMATION> {
    let mut startup_info: STARTUPINFOA = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOA>().try_into()?;
    let mut factorio_process_information: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let process_result = unsafe {
        CreateProcessA(
            factorio_path,
            PSTR::null(),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            PCSTR::null(),
            &startup_info,
            &mut factorio_process_information,
        )
    };

    if let Err(err) = process_result {
        bail!("Failed to create Factorio process: {err}");
    }

    let process_handle = factorio_process_information.hProcess;

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(process_handle).ok();
    }

    Ok(factorio_process_information)
}

fn main_detour(_argc: c_int, _argv: *const c_char, _envp: *const c_char) -> bool {
    info!("Detoured into LuaSurface.valid()!");
    let notepad_path = s!(r"C:/Program Files/WindowsApps/Microsoft.WindowsNotepad_11.2405.13.0_x64__8wekyb3d8bbwe/Notepad/Notepad.exe");

    start_factorio(notepad_path).unwrap();

    //unsafe { MessageBoxWHook.call(hwnd, text, replaced_caption, msgbox_style) }
    false
}

unsafe fn hook(pdb_cache: &PDBCache, function_name: &str) -> Result<()> {
    let Some(address) = pdb_cache.get_function_address(function_name) else {
        bail!("Failed to find main address");
    };

    info!("{} address: {:#x}", function_name, address);
    let target: FnMain = mem::transmute(address);
    MainHook.initialize(target, main_detour)?.enable()?;
    Ok(())
}

fn inject() -> Result<()> {
    let pdb_path = factorio_path("factorio.pdb")?;
    let cache = PDBCache::new(&pdb_path, "factorio.exe")?;

    let function_name = "?valid@LuaSurface@@UEBA_NXZ";
    unsafe { hook(&cache, function_name) }
}

fn start_steam() {
    let ip = "127.0.0.1:40267";
    let stream = TcpStream::connect(ip).unwrap_or_else(|_| {
        panic!("Could not establish stdout connection to rivets. Port {ip} is busy.")
    });
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();
}

#[ctor::ctor]
fn ctor() {
    start_steam();

    if let Err(e) = inject() {
        error!("{e}");
    }
}
