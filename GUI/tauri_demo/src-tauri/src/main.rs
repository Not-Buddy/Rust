// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, Manager, State};
use std::sync::Mutex;

struct FruitState {
    index: Mutex<usize>,
}

#[tauri::command]
fn get_fruit(state: State<FruitState>) -> Option<String> {
    let fruits = vec![
        "Apple", "Banana", "Cherry", "Mango", 
        "Pineapple", "Grape", "Orange", "Strawberry"
    ];
    
    let mut index = state.index.lock().unwrap();
    println!("[RUST] Button clicked - Current index: {}", *index);
    
    if *index < fruits.len() {
        let fruit = fruits[*index].to_string();
        *index += 1;
        println!("[RUST] Returning fruit: {}", fruit);
        Some(fruit)
    } else {
        println!("[RUST] No more fruits available!");
        None
    }
}

fn main() {
    println!("[RUST] Starting Tauri application...");
    
    Builder::default()
        .manage(FruitState {
            index: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![get_fruit])
        .setup(|app| {
            println!("[RUST] Application setup completed");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
