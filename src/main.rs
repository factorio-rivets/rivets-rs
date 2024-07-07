use dll_syringe::process::Process;
use dll_syringe::{process::OwnedProcess, Syringe};
use std::io;
use std::net::TcpListener;
use std::ptr::null_mut;
use windows::core::{s, PCSTR, PSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{
    SymCleanup, SymFromName, SymInitialize, SYMBOL_INFO,
};
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA,
};

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

fn kill_process(process: &str) {
    println!("Killing {} process...", process);

    loop {
        // find target process by name
        let target_process = OwnedProcess::find_first_by_name(process);

        match target_process {
            Some(process) => {
                match process.kill() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to kill process: {}", e);
                        return;
                    }
                }
                println!("Process killed successfully.");
            }
            None => return,
        }
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
            Some(null_mut()),
            Some(null_mut()),
            false,
            CREATE_SUSPENDED,
            Some(null_mut()),
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

fn get_symbol_address(
    factorio_process_information: PROCESS_INFORMATION,
    symbol_name: PCSTR,
) -> Option<*mut std::ffi::c_void> {
    unsafe {
        // Initialize the symbol handler for the current process.
        let process_handle: HANDLE = factorio_process_information.hProcess;

        // Allocate a buffer for the symbol information
        let mut buffer: Vec<u8> = vec![0; 1024];
        let sym_info = buffer.as_mut_ptr() as *mut SYMBOL_INFO;
        (*sym_info).SizeOfStruct = std::mem::size_of::<SYMBOL_INFO>() as u32;
        (*sym_info).MaxNameLen = 1024 - std::mem::size_of::<SYMBOL_INFO>() as u32;

        // Get the symbol information
        SymFromName(process_handle, symbol_name, sym_info).unwrap();

        let address = (*sym_info).Address as *mut std::ffi::c_void;
        Some(address)
    }
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

    kill_process("factorio");
    match start_factorio() {
        Ok(pi) => factorio_process_information = pi,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
    let process_handle = factorio_process_information.hProcess;

    inject_dll(&dll_path);
    unsafe { SymInitialize(process_handle, PCSTR::null(), true).unwrap() };

    let symbol_name = s!("?file_size@Filesystem@@YA_KAEBUPath@1@@Z"); // Mangled name
    let symbol_name_string = unsafe { symbol_name.to_string().unwrap() };

    match get_symbol_address(factorio_process_information, symbol_name) {
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
