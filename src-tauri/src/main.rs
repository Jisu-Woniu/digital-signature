// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use crate::{fs::detect_file_type, keygen::generate_key_pair, sign::sign_files};

mod error;
mod fs;
mod keygen;
mod sign;

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_key_pair,
            detect_file_type,
            sign_files
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
