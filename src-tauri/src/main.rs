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

#[tauri::command]
fn get_turn_owner(state :State<GameManager>) -> [u16; 2] {
    let turn_owner = state.inner().get_turn_owner();
    return turn_owner;
}

#[tauri::command]
fn pass_turn(state :State<GameManager>){
    
    state.inner().pass_turn();
}


fn main() {
    
    tauri::Builder::default()
    .manage(GameManager::mock_gamemanager())
        .invoke_handler(tauri::generate_handler![get_board,
                                                 get_turn_owner,
                                                 pass_turn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


