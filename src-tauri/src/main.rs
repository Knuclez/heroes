#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod gameplay_classes;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_game() -> gameplay_classes::GameManager {
   let gm = gameplay_classes::GameManager::mock_gamemanager(); 
    return gm;
}

fn main() {
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


