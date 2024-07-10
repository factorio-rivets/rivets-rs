use dll_syringe::{process::OwnedProcess, Syringe};
use std::io;
use std::net::TcpListener;
use windows::core::{s, PCSTR, PSTR};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA,
};
use anyhow::{bail, anyhow, Result};

fn inject_dll(dll_name: &str) -> Result<()> {
    println!("Injecting DLL into Factorio process...");
    let Some(process) = OwnedProcess::find_first_by_name("factorio") else {
        bail!("Factorio process not found.");
    };

    let syringe = Syringe::for_process(process);
    let option = syringe.inject(dll_name);

    match option {
        Ok(_) => Ok(()),
        Err(e) => bail!("Failed to inject DLL: {e}"),
    }
}

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

    Ok(factorio_process_information)
}

fn main() -> Result<()> {
    let dll_path = r"target\debug\examplemod.dll";

    let listener = TcpListener::bind("127.0.0.1:40267");
    let listener = listener.map_err(|e| {
        anyhow!("Failed to copy the Factorio output logs. Is rivets already running?\n{e}")
    })?;

    let factorio_path = s!(r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.exe");
    let factorio_process_information: PROCESS_INFORMATION = start_factorio(factorio_path)?;
    let process_handle = factorio_process_information.hProcess;

    inject_dll(dll_path)?;
    println!("DLL injected successfully.");

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(process_handle).ok();
    }

    // Duplicate the factorio stdout stream onto our own stdout.
    #[allow(clippy::expect_used)]
    io::copy(&mut listener.incoming().next().expect("Factorio will always have stdout.")?, &mut io::stdout())?;

    Ok(())
}
