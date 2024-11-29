use serde::Serialize;
use std::{path::Path, process::Command};

/// A struct that represents an executable that can be run.
/// # Fields
/// * `id` - An optional integer that represents the ID of the executable.
/// * `name` - A string that holds the name of the executable.
/// * `path` - A string that holds the path to the executable.
#[derive(Serialize)]
struct Executable {
    id: Option<i32>,
    name: String,
    path: String,
}

/// Checks if the provided path is valid.
///
/// This function takes a string slice representing a file or directory path
/// and checks if it exists on the filesystem. It returns `true` if the path
/// exists, otherwise `false`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to be checked.
///
/// # Returns
///
/// * `bool` - `true` if the path exists, `false` otherwise.
fn check_for_valid_path(path: &str) -> bool {
    Path::new(path).exists()
}


/// This function returns a list of executables that can be run.
/// # Returns
/// * `Vec<Executable>` - A vector of `Executable` objects.
#[tauri::command]
fn get_executables() -> Result<Vec<Executable>, String> {
    let mut executables = Vec::new();
    executables.push(Executable {
        id: Option::from(1),
        name: "Google Chrome".to_string(),
        path: "".to_string(),
    });


    Ok(executables)
}


/// This function attempts to execute the executable stored on  `path`.
/// If the execution fails, it will panic with an appropriate message.
///
/// # Returns
///
/// * `u32` - The process ID of the spawned executable.
#[tauri::command]
fn run_executable(path: String) -> Result<u32, String> {
    if !check_for_valid_path(&path) {
        return Err("The path is not valid.".to_string());
    }

    let status = Command::new(path).spawn().expect("The executable should run.");

    Ok(status.id())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_executables, run_executable
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
