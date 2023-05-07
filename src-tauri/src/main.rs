// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess;
use chess::*;
use serde::Deserialize;
use std::sync::Mutex;
use tauri::State;

struct BoardState(pub Mutex<Board>);

#[derive(Deserialize, Debug)]
struct Coord {
    r: usize,
    f: usize,
}

#[tauri::command]
fn get_board(board: State<BoardState>) -> Board {
    board.0.lock().unwrap().clone()
}

#[tauri::command]
fn get_moves(board: State<BoardState>, from: Coord) -> Vec<(usize, usize)> {
    Board::get_legal_moves(&mut board.0.lock().unwrap(), from.r, from.f)
}

#[tauri::command]
fn make_move(
    board: State<BoardState>,
    moves: Vec<(usize, usize)>,
    from: Coord,
    to: Coord,
) -> Board {
    Board::make_move(
        &mut board.0.lock().unwrap(),
        moves,
        from.r,
        from.f,
        to.r,
        to.f,
    );
    board.0.lock().unwrap().clone()
}

fn main() {
    tauri::Builder::default()
        .manage(BoardState(Mutex::new(Board::new())))
        .invoke_handler(tauri::generate_handler![get_board, get_moves, make_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
