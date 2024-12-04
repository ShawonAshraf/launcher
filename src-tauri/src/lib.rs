mod database;

use crate::database::{Executable, Integration};
use std::{path::Path, process::Command};
use dirs;

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
    // create the database file in the user document directory
    let doc_dir = dirs::document_dir().unwrap();
    let db_path = doc_dir.join("exedb.db")
        .to_str().unwrap().to_string();
    
    let integration = Integration::new(db_path).unwrap();
    integration.create_table();
    let executables = integration.list_all();

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


/// This function deletes an executable from the database.
/// # Arguments
/// * `id` - A string slice that holds the ID of the executable to be deleted.
#[tauri::command]
fn delete_executable(id: String) {
    let integration = Integration::new("exedb.db".to_string()).unwrap();
    integration.delete_exe(id);
}

/// This function adds an executable to the database.
/// # Arguments
/// * `name` - A string slice that holds the name of the executable.
/// * `path` - A string slice that holds the path of the executable.
#[tauri::command]
fn add_executable(name: String, path: String) {
    let integration = Integration::new("exedb.db".to_string()).unwrap();
    let new_exe = Executable { id: None, name, path };
    integration.add_exe(new_exe);
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_executables, 
            run_executable, 
            delete_executable, 
            add_executable
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
