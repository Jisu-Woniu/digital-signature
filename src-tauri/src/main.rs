// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use crate::{
    fs::detect_file_type, keygen::generate_key_pair, sign::sign_files, verify::verify_signatures,
};

mod error;
mod fs;
mod keygen;
mod sign;
mod verify;

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_key_pair,
            detect_file_type,
            sign_files,
            verify_signatures
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
