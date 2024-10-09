#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler, Builder};

use crate::{
    fs::{detect_file_type, get_file_names},
    keygen::generate_key_pair,
    sign::sign_files,
    verify::verify_signatures,
};

mod error;
mod fs;
mod keygen;
mod sign;
mod verify;

fn main() -> anyhow::Result<()> {
    #[cfg(all(debug_assertions, feature = "devtools"))]
    let builder = Builder::default().plugin(tauri_plugin_devtools::init());
    #[cfg(not(all(debug_assertions, feature = "devtools")))]
    let builder = Builder::default();

    builder
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(generate_handler![
            generate_key_pair,
            detect_file_type,
            sign_files,
            verify_signatures,
            get_file_names
        ])
        .run(generate_context!())?;
    Ok(())
}
