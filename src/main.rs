use dll_syringe::process::Process;
use dll_syringe::{Syringe, process::OwnedProcess};
use std::ffi::CString;
use std::net::TcpListener;
use std::ptr::null_mut;
use winapi::um::processthreadsapi::{CreateProcessA, ResumeThread, PROCESS_INFORMATION, STARTUPINFOA};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winbase::CREATE_SUSPENDED;
use std::io;

fn inject_dll(dll_name: &str) {
    println!("Injecting DLL into Factorio process...");
    let target_process = OwnedProcess::find_first_by_name("factorio").expect("Failed to find Factorio process.");
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
    let factorio_path = r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.exe";
    let factorio_exe = CString::new(factorio_path).expect("CString::new failed");

    let mut si: STARTUPINFOA = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOA>() as u32;
    let mut factorio_process_information: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

    println!("Creating factorio.exe process...");
    let error_code = unsafe {
        CreateProcessA(
            factorio_exe.as_ptr(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            CREATE_SUSPENDED,
            null_mut(),
            null_mut(),
            &mut si,
            &mut factorio_process_information,
        )
    };

    return if error_code == 0 {
        Err("Failed to start the process.".to_string())
    } else {
        Ok(factorio_process_information)
    };
}

fn main() {
    let dll_path = r"target\debug\examplemod.dll";

    let listener = match TcpListener::bind("127.0.55.1:16337") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to copy the Factorio output logs. Is rivets already running?\n{}", e);
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

    inject_dll(&dll_path);

    unsafe { ResumeThread(factorio_process_information.hThread); }
    unsafe { CloseHandle(factorio_process_information.hThread); }
    unsafe { CloseHandle(factorio_process_information.hProcess); }

    // Duplicate the factorio stdout stream onto our own stdout.
    io::copy(&mut listener.incoming().next().unwrap().unwrap(), &mut io::stdout()).unwrap();
}