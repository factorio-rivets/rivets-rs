//! A small windows application to inject the Rivets DLL into Factorio.

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use dll_syringe::process::{BorrowedProcess, ProcessModule};
use dll_syringe::{process::OwnedProcess, Syringe};
use semver::Version;
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::{self};
use std::os::windows::io::FromRawHandle;
use std::path::{Path, PathBuf};
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::{
    CloseHandle, SetHandleInformation, BOOL, HANDLE, HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE,
};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Pipes::CreatePipe;
use windows::Win32::System::Threading::{
    CreateProcessA, ResumeThread, TerminateProcess, CREATE_SUSPENDED, PROCESS_INFORMATION,
    STARTUPINFOA,
};
use zip::read::ZipArchive;

fn unzip_specific_file(zip_path: &Path, file_name: &str, output_path: &Path) -> Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file).context("Failed to open rivets mod ZIP archive")?;

    let mut zip_file = archive
        .by_name(file_name)
        .context("Failed to find rivets.dll in ZIP archive")?;

    let err = format!(
        "Failed to create output file {}",
        output_path
            .to_str()
            .ok_or(anyhow!("Failed to unzip rivets.dll"))?
    );
    let mut output_file = File::create(output_path).context(err)?;

    io::copy(&mut zip_file, &mut output_file)
        .context("Failed to write ZIP entry to output file")?;

    Ok(())
}

fn find_latest_rivets_version(mods_folder: &Path) -> Result<String> {
    let mut latest_version: Option<Version> = None;
    let mut latest_version_file: Option<PathBuf> = None;

    for entry in fs::read_dir(mods_folder)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Some(file_name_str) = file_name.to_str() {
            if file_name_str.starts_with("rivets_") && file_name_str.ends_with(".zip") {
                let version_str = file_name_str
                    .trim_start_matches("rivets_")
                    .trim_end_matches(".zip");
                if let Ok(version) = Version::parse(version_str) {
                    if let Some(current_version) = &latest_version {
                        if version > *current_version {
                            latest_version = Some(version);
                            latest_version_file = Some(entry.path());
                        }
                    } else {
                        latest_version = Some(version);
                        latest_version_file = Some(entry.path());
                    }
                }
            }
        }
    }

    if let Some(file_path) = latest_version_file {
        if let Some(file_name) = file_path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                return Ok(file_name_str.to_string());
            }
        }
    }

    Err(anyhow!("No rivets mod found in the mods folder"))
}

fn extract_dll(mods_folder: &Path) -> Result<PathBuf> {
    const DLL_NAME: &str = "rivets/rivets.dll";

    let latest_rivets_version = find_latest_rivets_version(mods_folder)?;
    println!("Found rivets version: {latest_rivets_version} Injecting...",);

    let mut output_path = std::env::current_dir()?;
    output_path.push(format!("{latest_rivets_version}.dll"));

    let zip_path = mods_folder.join(latest_rivets_version);
    let dll_path = mods_folder.join(DLL_NAME);

    unzip_specific_file(&zip_path, DLL_NAME, &output_path)?;

    Ok(dll_path)
}

fn get_syringe() -> Result<Syringe> {
    let Some(process) = OwnedProcess::find_first_by_name("factorio") else {
        bail!("Factorio process not found.");
    };

    Ok(Syringe::for_process(process))
}

fn inject_dll<'a>(
    syringe: &'a Syringe,
    dll_path: &Path,
) -> Result<ProcessModule<BorrowedProcess<'a>>> {
    println!("Injecting DLL into Factorio process...");

    syringe
        .inject(dll_path)
        .map_err(|e| anyhow!("Failed to inject DLL: {e}"))
}

fn create_pipe() -> Result<(HANDLE, HANDLE)> {
    let mut stdout_read = INVALID_HANDLE_VALUE;
    let mut stdout_write = INVALID_HANDLE_VALUE;
    let sa = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        bInheritHandle: BOOL(1), // The child process needs to inherit the handle
        lpSecurityDescriptor: std::ptr::null_mut(),
    };
    unsafe {
        CreatePipe(&mut stdout_read, &mut stdout_write, Some(&sa), 0)?;
        SetHandleInformation(stdout_read, 0, HANDLE_FLAG_INHERIT)?;
    }

    Ok((stdout_read, stdout_write))
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
    mods_folder: PathBuf,
}

fn main() -> Result<()> {
    let (stdout_read, _) = create_pipe()?;
    let mut reader = unsafe { std::fs::File::from_raw_handle(stdout_read.0) };

    let cli = Cli::parse();

    let mut factorio_path = std::env::current_dir()?;
    factorio_path.push("factorio.exe");

    let factorio_path = CString::new(factorio_path.as_os_str().to_string_lossy().into_owned())?;
    println!("Factorio path: {factorio_path:?}");
    let factorio_path = PCSTR(factorio_path.as_ptr().cast());

    let dll_path = extract_dll(&cli.mods_folder)?;

    let factorio_process_information: PROCESS_INFORMATION = start_factorio(factorio_path)?;
    println!("Factorio process started.");
    let syringe = get_syringe().inspect_err(|_| {
        attempt_kill_factorio(factorio_process_information);
    })?;
    inject_dll(&syringe, &dll_path)?;
    println!("DLL injected successfully.");

    unsafe {
        ResumeThread(factorio_process_information.hThread);
        CloseHandle(factorio_process_information.hThread).ok();
        CloseHandle(factorio_process_information.hProcess).ok();
    }

    // Duplicate the factorio stdout stream onto our own stdout using OS pipes.
    io::copy(&mut reader, &mut io::stdout())?;

    Ok(())
}

fn attempt_kill_factorio(factorio_process_information: PROCESS_INFORMATION) {
    if let Err(e) = unsafe { TerminateProcess(factorio_process_information.hProcess, 0) } {
        eprintln!("Failed to terminate Factorio process after experiencing a rivets error: {e}");
        eprintln!("You likely have a ghost Factorio process running. Please kill it manually via task manager.");
    }
}
