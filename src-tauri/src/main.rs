#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use tauri::{command, CustomMenuItem, SystemTray, SystemTrayEvent, Menu, MenuItem, Submenu, Window, WindowEvent};
use tauri::{command, Window};


use std::process::Command;

#[command]
fn open_in_edge(url: String) -> Result<(), String> {
  if cfg!(target_os = "windows") {
      Command::new("cmd")
          .args(&["/C", "start", "msedge", &url])
          .output()
          .map_err(|err| err.to_string())?;
      Ok(())
  } else {
      Err("This command is only available on Windows.".to_string())
  }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![open_in_edge])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}