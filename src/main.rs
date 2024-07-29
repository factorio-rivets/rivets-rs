use anyhow::{anyhow, bail, Result};
use dll_syringe::process::{BorrowedProcess, ProcessModule};
use dll_syringe::{process::OwnedProcess, Syringe};
use std::ffi::CString;
use std::io;
use std::net::TcpListener;
use std::path::Path;
use traits::{factorio_path, AsPcstr};
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA,
};

mod luastate;
mod pdb2hpp;
mod traits;

fn get_syringe() -> Result<Syringe> {
    let Some(process) = OwnedProcess::find_first_by_name("factorio") else {
        bail!("Factorio process not found.");
    };

    Ok(Syringe::for_process(process))
}

fn inject_dll<'a>(
    syringe: &'a Syringe,
    dll_name: &Path,
) -> Result<ProcessModule<BorrowedProcess<'a>>> {
    println!("Injecting DLL into Factorio process...");

    syringe
        .inject(dll_name)
        .map_err(|e| anyhow!("Failed to inject DLL: {e}"))
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

fn inject() -> Result<()> {
    let dll_path = Path::new("./target/debug/achievement_enabler.dll");

    let listener = TcpListener::bind("127.0.0.1:40267");
    let listener = listener.map_err(|e| {
        anyhow!("Failed to copy the Factorio output logs. Is rivets already running?\n{e}")
    })?;

    let factorio_path = CString::new(
        factorio_path("factorio.exe")?
            .as_os_str()
            .to_string_lossy()
            .into_owned(),
    )?;
    println!("Factorio path: {factorio_path:?}");
    let factorio_process_information: PROCESS_INFORMATION =
        start_factorio(factorio_path.as_pcstr())?;
    let process_handle = factorio_process_information.hProcess;

    let syringe = get_syringe()?;
    inject_dll(&syringe, dll_path)?;
    println!("DLL injected successfully.");

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(process_handle).ok();
    }

    // Duplicate the factorio stdout stream onto our own stdout.
    io::copy(
        &mut listener
            .incoming()
            .next()
            .expect("Factorio will always have stdout.")?,
        &mut io::stdout(),
    )?;

    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().any(|x| x == *"bindings") {
        let pdb_path = factorio_path("factorio.pdb")?;
        pdb2hpp::generate(&pdb_path)?;
    } else {
        inject()?;
    }

    Ok(())
}
