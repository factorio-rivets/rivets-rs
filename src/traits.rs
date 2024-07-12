use anyhow::{bail, Result};
use std::{ffi::CStr, path::PathBuf};

use dirs::home_dir;
use windows::core::PCSTR;

pub trait AsPcstr {
    fn as_pcstr(&self) -> PCSTR;
}

impl AsPcstr for CStr {
    fn as_pcstr(&self) -> PCSTR {
        PCSTR(self.as_ptr().cast())
    }
}

pub fn factorio_path(filename: &str) -> Result<PathBuf> {
    let factorio_path = home_dir();

    if let Some(mut path) = factorio_path {
        path.push(r"Documents\factorio\bin\x64\");
        path.push(filename);
        Ok(path)
    } else {
        bail!("Failed to find the user's home directory.")
    }
}
