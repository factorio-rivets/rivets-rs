#![cfg(all(windows))]

use std::{net::TcpStream, sync::Mutex};
use tracing::info;

use ctor::ctor;

#[ctor]
fn ctor() {
    let stream = TcpStream::connect("127.0.55.1:16337").unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();
    for _ in 0..10 {
        info!("Hi!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }
}
