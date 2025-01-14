
use std::io::Write;
use draw_utils::{draw_horizontal_line, draw_vertical_line, set_to_blank};
use handlers::{handle_lost, handle_menu, handle_playing, handle_turorial, handle_won};
use winapi_util::console::Console;
mod extra;
mod draw_utils;
mod handlers;

pub struct Canva {
    x: i32,
    y: i32,
    xu: usize,
    yu: usize
}

pub const CANVA: Canva = Canva{
    x: 200, 
    y: 40, 
    xu: 200, 
    yu: 40
};

// CANVA.xu and CANVA.yu are meant to be the same values, but for convinience reasons they are split in i32 and usize
fn main() {

    let mut array: [[char; CANVA.yu]; CANVA.xu] = [[' '; CANVA.yu]; CANVA.xu];
    let mut game_state: GameState = GameState::Menu;
    let mut points: [i32; 1] = [0];

    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }

    println!("\x1b[?25l");
    println!("\x1b[=0h");
    
    loop { //game main loop 

        // let area_dica: usize = X - X / 4;
        match game_state {
            GameState::Menu => set_to_blank(&mut array),
            GameState::Turorial => set_to_blank(&mut array),
            GameState::Playing => set_to_blank(&mut array),
            GameState::Won => {},
            GameState::Lost => {},
            GameState::Quit => {break;}, //stops the main loop
        }
       

        // This block makes the '|' at the sides of the canva
        draw_vertical_line(0, CANVA.y - 1, 0, &mut array, '|');
        draw_vertical_line(0, CANVA.y - 1, 1, &mut array, '|');
        draw_vertical_line(0, CANVA.y - 1, CANVA.x - 1, &mut array, '|');
        draw_vertical_line(0, CANVA.y -1, CANVA.x - 2, &mut array, '|');

        // And this one makes the '=' at the top and the botton of the space
        draw_horizontal_line(0, CANVA.x, 1, &mut array, '=');
        draw_horizontal_line(0, CANVA.x, CANVA.y - 2, &mut array, '=');
        draw_horizontal_line(0, CANVA.x, 0, &mut array, '=');
        draw_horizontal_line(0, CANVA.x, CANVA.y - 1, &mut array, '=');

        // OBS: due to the lines at the sides of the canva, the working space is X: 2..X-2 and Y: 2..Y-2
 
        match game_state {
            GameState::Menu => handle_menu(&mut array, &mut game_state),
            GameState::Turorial => handle_turorial(&mut array, &mut game_state),
            GameState::Playing => handle_playing(&mut array, &mut game_state, &mut points),
            GameState::Won => handle_won(&mut array, &mut game_state),
            GameState::Lost => handle_lost(&mut array, &mut game_state),
            GameState::Quit => {break;}, //stops the main loop
        }
    
    }
}

enum GameState {
    Menu,
    Turorial,
    Playing,
    Won,
    Lost,
    Quit

}

fn str_to_char (string: &str) -> Vec<char> {

    let mut char_vec = Vec::new();

    for character in string.chars() {
        char_vec.push(character)
    }

    char_vec

}

// this fn only returns the firs char (if the user writes more than one) because this games only requires one char as input
fn get_input() -> char {

    let mut input: String = String::new();
    let mut temp_vec: Vec<char> = Vec::new();
    input.clear();

     std::io::stdout().flush().unwrap();
    
    loop {
        
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim().is_empty() {
            input.clear();
        } else {break}
        
    }
    
    for el in input.trim().chars() {
        temp_vec.push(el);
    }
        
    temp_vec[0] 

    
}













