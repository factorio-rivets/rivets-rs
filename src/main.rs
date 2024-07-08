#![allow(unused_imports, unused_variables, dead_code)]

use dll_syringe::process::Process;
use dll_syringe::{process::OwnedProcess, Syringe};
use std::ffi::{c_void, CStr};
use std::io;
use std::net::TcpListener;
use std::ptr::{null, null_mut};
use windows::core::{s, Error, PCSTR, PSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, BOOL, GENERIC_READ, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{
    SymCleanup, SymEnumLines, SymEnumSymbols, SymEnumerateModules64, SymFromAddr, SymFromName,
    SymGetLineFromAddr64, SymInitialize, SymLoadModuleEx, IMAGEHLP_LINE64,
    PSYM_ENUMERATESYMBOLS_CALLBACK, PSYM_ENUMLINES_CALLBACK, SRCCODEINFO, SYMBOL_INFO,
    SYM_LOAD_FLAGS,
};
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA,
};
use windows::Win32::Storage::FileSystem::{CreateFileA, FILE_SHARE_READ, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL};

fn inject_dll(dll_name: &str) {
    println!("Injecting DLL into Factorio process...");
    let target_process =
        OwnedProcess::find_first_by_name("factorio").expect("Failed to find Factorio process.");
    let syringe = Syringe::for_process(target_process);
    let option = syringe.inject(dll_name);

    match option {
        Ok(_) => println!("DLL injected successfully."),
        Err(e) => panic!("Failed to inject DLL: {}", e),
    }
}

fn start_factorio() -> Result<PROCESS_INFORMATION, String> {
    let factorio_path = s!(r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.exe");

    let mut startup_info: STARTUPINFOA = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;
    let mut factorio_process_information: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

    println!("Creating factorio.exe process...");
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
            &mut startup_info,
            &mut factorio_process_information,
        )
    };

    if let Err(err) = process_result {
        return Err(format!("Failed to create Factorio process: {}", err));
    }

    println!("Factorio process created successfully.");
    Ok(factorio_process_information)
}

fn get_symbol_info_name(symbol_info: &SYMBOL_INFO) -> String {
    let string_length = symbol_info.NameLen as usize;
    let name_ptr = (*symbol_info).Name.as_ptr() as *const u8;
    let name_slice = unsafe { std::slice::from_raw_parts(name_ptr, string_length) };
    let name = unsafe { CStr::from_bytes_with_nul_unchecked(name_slice) };
    name.to_string_lossy().to_string()
}

fn print_symbol_info(psyminfo: *const SYMBOL_INFO) {
    let i = unsafe { *psyminfo };
    let name: String = get_symbol_info_name(&i);
    println!("Symbol: SizeOfStruct={:?}, MaxNameLen={:?}, Address={:?}, TypeIndex={:?}, Name={:?}, ModBase={:?}, Flags={:?}, NameLen={:?}, Tag={:?}, Scope={:?}, Register={:?}",
        i.SizeOfStruct, i.MaxNameLen, i.Address-i.ModBase, i.TypeIndex, name, i.ModBase, i.Flags, i.NameLen, i.Tag, i.Scope, i.Register);
    println!("len of name={:?}", name.len());
}

unsafe extern "system" fn cb(
    psyminfo: *const SYMBOL_INFO,
    symbolsize: u32,
    usercontext: *const core::ffi::c_void,
) -> BOOL {
    print_symbol_info(psyminfo);
    true.into()
}

/*unsafe extern "system" fn cb(lineinfo: *const SRCCODEINFO, usercontext: *const core::ffi::c_void) -> BOOL {
    println!("Line: {:?}, {:?}", lineinfo, usercontext);
    true.into()
}*/

static mut MODULE_NAME: &'static str = "factorio";

fn get_dll_base_address(
    module_name: &'static str,
    process_information: PROCESS_INFORMATION,
) -> Result<u64, String> {
    static mut BASE_ADRESS: Option<u64> = None;
    unsafe { MODULE_NAME = module_name };

    unsafe extern "system" fn callback(
        modulename: PCSTR,
        baseofdll: u64,
        usercontext: *const c_void,
    ) -> BOOL {
        let module_name = unsafe { modulename.to_string().unwrap() };
        println!("Module: {:?}, {:?}", module_name, baseofdll);
        if module_name.contains(MODULE_NAME) {
            BASE_ADRESS = Some(baseofdll);
            //false
            true
        } else {
            true
        }
        .into()
    }

    let process_handle = process_information.hProcess;
    unsafe {
        SymEnumerateModules64(process_handle, Some(callback), None).unwrap();
    }

    match unsafe { BASE_ADRESS } {
        Some(base_address) => Ok(base_address),
        None => Err("Failed to get Factorio base address.".to_owned()),
    }
}

fn get_symbol_address(
    factorio_process_information: PROCESS_INFORMATION,
    symbol_name: PCSTR,
    base_address: u64,
) -> Option<*mut std::ffi::c_void> {
    unsafe {
        // Initialize the symbol handler for the current process.
        let process_handle: HANDLE = factorio_process_information.hProcess;

        // Allocate a buffer for the symbol information
        let buffer_size = std::mem::size_of::<SYMBOL_INFO>() as u32;
        let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
        let sym_info: *mut SYMBOL_INFO = buffer.as_mut_ptr() as *mut SYMBOL_INFO;
        (*sym_info).SizeOfStruct = buffer_size;
        (*sym_info).MaxNameLen = 1024 - buffer_size;
        //(*sym_info).ModBase = base_address;

        // Get the symbol information
        //SymFromName(process_handle, symbol_name, sym_info).unwrap();

        //SymFromAddr(process_handle, base_address+1000000, None, sym_info).unwrap();
        //print_symbol_info(sym_info);

        SymEnumSymbols(process_handle, base_address, s!("*!*"), Some(cb), None).unwrap();

        //SymEnumLines(process_handle, 0, None, None, Some(cb), None).unwrap();

        //let address = (*sym_info).Address as *mut std::ffi::c_void;
        Some(1 as *mut std::ffi::c_void)
    }
}

fn initialize_symbol_handler(process_handle: HANDLE, base_address: u64) -> Result<(), String> {
    let pdb_file_path = s!(r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb");
    unsafe {
        SymInitialize(process_handle, PCSTR::null(), true).map_err(|e| format!("{}", e))?;

        /*let file_handler: HANDLE = CreateFileA(
            pdb_file_path,
            GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        ).map_err(|e| format!("{}", e))?;*/

        let load_pdb_result = SymLoadModuleEx(
            process_handle,
            None,
            pdb_file_path,
            PSTR::null(),
            base_address,
            0,
            None,
            SYM_LOAD_FLAGS(0),
        );
        
        if load_pdb_result == 0 {
            let error = GetLastError();
            return Err(format!("Failed to load Factorio debug symbols. {:?}", error));
        }
    };

    Ok(())
}

fn main() {
    let dll_path = r"target\debug\examplemod.dll";

    let listener = match TcpListener::bind("127.0.55.1:16337") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!(
                "Failed to copy the Factorio output logs. Is rivets already running?\n{}",
                e
            );
            return;
        }
    };

    let factorio_process_information;

    match start_factorio() {
        Ok(pi) => factorio_process_information = pi,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
    let process_handle = factorio_process_information.hProcess;

    inject_dll(&dll_path);

    let base_address = get_dll_base_address("factorio", factorio_process_information).unwrap();
    println!("Factorio base address: {:?}", base_address);

    initialize_symbol_handler(process_handle, base_address).unwrap();
    println!("Factorio debug symbols loaded successfully.");

    let symbol_name = s!("main"); // Mangled name
    let symbol_name_string = unsafe { symbol_name.to_string().unwrap() };

    match get_symbol_address(factorio_process_information, symbol_name, base_address) {
        Some(address) => println!("Address of {}: {:?}", symbol_name_string, address),
        None => println!("Symbol not found"),
    }

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(process_handle).ok();
        SymCleanup(process_handle).ok();
    }

    // Duplicate the factorio stdout stream onto our own stdout.
    io::copy(
        &mut listener.incoming().next().unwrap().unwrap(),
        &mut io::stdout(),
    )
    .unwrap();
}
