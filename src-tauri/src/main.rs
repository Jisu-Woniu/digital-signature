// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use crate::{fs::file_type, keygen::generate_key_pair};

mod error;
mod fs;
mod keygen;

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_key_pair, file_type])
        .run(tauri::generate_context!())?;
    Ok(())
}
