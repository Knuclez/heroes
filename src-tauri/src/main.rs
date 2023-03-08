#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gameplay_classes::GameManager;
use gameplay_classes::PlayablePiece;
use tauri::State;

mod gameplay_classes;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_board(state :State<GameManager>) -> [[PlayablePiece; 12]; 6] {
    return state.inner().get_board().clone();
}

fn main() {
    
    tauri::Builder::default()
    .manage(GameManager::mock_gamemanager())
        .invoke_handler(tauri::generate_handler![get_board])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


