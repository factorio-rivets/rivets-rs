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
    // find target process by name
    let target_process = OwnedProcess::find_first_by_name("factorio").unwrap();

    // create a new syringe for the target process
    let syringe = Syringe::for_process(target_process);

    // inject the payload into the target process
    let option = syringe.inject(dll_name);

    match option {
        Ok(_) => println!("DLL injected successfully."),
        Err(e) => panic!("Failed to inject DLL: {}", e),
    }
}

fn kill_factorio() {
    println!("Killing Factorio process...");

    loop {
        // find target process by name
        let target_process = OwnedProcess::find_first_by_name("factorio");

        match target_process {
            Some(process) => {
                match process.kill() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to kill Factorio process: {}", e);
                        return;
                    }
                }
                println!("Factorio process killed successfully.");
            }
            None => return,
        }
    }
}

fn main() {
    let dll_path = r"target\debug\examplemod.dll";
    let factorio_path = r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.exe";
    let factorio_exe = CString::new(factorio_path).expect("CString::new failed");

    let mut si: STARTUPINFOA = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOA>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

    let listener = TcpListener::bind("127.0.1.1:16337").unwrap();

    let success = unsafe {
        kill_factorio();
        println!("Creating process...");
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
            &mut pi,
        )
    };

    if success == 0 {
        eprintln!("Failed to start the process.");
        return;
    } else {
        println!("Process created successfully.");
    }

    inject_dll(&dll_path);

    unsafe { ResumeThread(pi.hThread); }
    unsafe { CloseHandle(pi.hThread); }
    unsafe { CloseHandle(pi.hProcess); }

    io::copy(&mut listener.incoming().next().unwrap().unwrap(), &mut io::stdout()).unwrap();
}