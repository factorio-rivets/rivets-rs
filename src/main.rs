use dll_syringe::{process::OwnedProcess, Syringe};
use std::io;
use std::net::TcpListener;
use windows::core::{s, PCSTR, PSTR};
use windows::Win32::Foundation::CloseHandle;
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

fn start_factorio(factorio_path: PCSTR) -> Result<PROCESS_INFORMATION, String> {
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

    let factorio_path = s!(r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.exe");
    let factorio_process_information: PROCESS_INFORMATION;

    match start_factorio(factorio_path) {
        Ok(pi) => factorio_process_information = pi,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
    let process_handle = factorio_process_information.hProcess;

    inject_dll(&dll_path);

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(process_handle).ok();
    }

    // Duplicate the factorio stdout stream onto our own stdout.
    let _ = io::copy(
        &mut listener.incoming().next().unwrap().unwrap(),
        &mut io::stdout(),
    );
}
