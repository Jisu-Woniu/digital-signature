// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            generate_key_pair,
            detect_file_type,
            sign_files,
            verify_signatures,
            get_file_names
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
