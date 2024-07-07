#![cfg(all(windows))]
//! A `MessageBoxW` detour example.
//!
//! Ensure the crate is compiled as a 'cdylib' library to allow C interop.
use std::{net::TcpStream, sync::Mutex};
use tracing::info;

use ctor::ctor;

#[ctor]
fn ctor() {
    let stream = TcpStream::connect("127.0.1.1:16337").unwrap();
    tracing_subscriber::fmt::fmt().with_writer(Mutex::new(stream)).init();
    for _ in 0..10 {
        info!("Hi!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }
}