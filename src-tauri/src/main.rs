// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::result::Result;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Task {
    id: u32,
    text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Tasks {
    tasks: Vec<Task>,
}

#[tauri::command]
fn write_to_file(tasks: String) -> Result<(), String> {
    println!("Writing to file: {}", tasks);
    if tasks.is_empty() {
        return Ok(());
    }
    let mut file = File::create("C:/Users/goran/projects/todo app/data/todoList.json")
        .map_err(|e| e.to_string())?;
    file.write_all(tasks.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn read_from_file() -> Result<Vec<Task>, String> {
    if !Path::new("C:/Users/goran/projects/todo app/data/todoList.json").exists() {
        return Ok(Vec::new());
    }
    let file = File::open("C:/Users/goran/projects/todo app/data/todoList.json")
        .map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let tasks: Tasks =
        serde_json::from_reader(reader).unwrap_or_else(|_| Tasks { tasks: Vec::new() });
    println!("Reading from file: {:?}", tasks.tasks);
    Ok(tasks.tasks)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_to_file, read_from_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
