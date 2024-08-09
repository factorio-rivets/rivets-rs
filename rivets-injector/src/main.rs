//! A small windows application to inject the Rivets DLL into Factorio.

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use dll_syringe::process::{BorrowedProcess, ProcessModule};
use dll_syringe::{process::OwnedProcess, Syringe};
use std::ffi::CString;
use std::path::{Path, PathBuf};
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA,
};

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

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    factorio_path: PathBuf,
    dll_path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let factorio_path = CString::new(cli.factorio_path.as_os_str().to_string_lossy().into_owned())?;
    println!("Factorio path: {factorio_path:?}");
    let factorio_path = PCSTR(factorio_path.as_ptr().cast());

    let factorio_process_information: PROCESS_INFORMATION = start_factorio(factorio_path)?;
    let process_handle = factorio_process_information.hProcess;

    let syringe = get_syringe()?;
    inject_dll(&syringe, &cli.dll_path)?;
    println!("DLL injected successfully.");

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(process_handle).ok();
    }

    Ok(())
}
