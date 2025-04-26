/*  This is the main game file */
/*  Last updated: 25/04/2025 */
/*  Chatgame intended for windows, not sure if it works on linux */
/*  This game seems to not work properly with windows 11 ! */

//
//-------------------------- Crates declaration --------------------------//
//
use dutils::{Canva, Point, Cursor};
use handlers::{handle_lost, handle_menu, handle_playing, handle_turorial, handle_won};
use std::io::Write;
mod dutils;
mod extra;
mod handlers;

//-------------------------- Enum of the main state machine --------------------------//
enum GameState {
    Menu,
    Turorial,
    Playing,
    Won,
    Lost,
    Quit,
}

//-------------------------- Main game loop --------------------------//
fn main() {
    let mycursor: Cursor = Cursor::new(false, false);       //Creates an instance of the Cursor struct
    let canva: Canva = Canva::new(200, 40, mycursor); //Create an instance of the Canva struct

    let mut game_state: GameState = GameState::Menu;
    let mut points: u8 = 0;

    loop {
        /*  This is the main game loop.
         *  Erases the canvas when the state changes to Menu, Tutorial or Playing.
         *  Draws the borders of the valid game area
         *  Calls the handlers for each state of the state machine
         *  The loop stops when the state is changed to 'Quit'
         */

        match game_state {
            GameState::Menu => canva.set_to_blank(),
            GameState::Turorial => canva.set_to_blank(),
            GameState::Playing => canva.set_to_blank(),
            GameState::Won => {}
            GameState::Lost => {}
            GameState::Quit => {break;}                     //stops the main loop} 
        }

        // This block makes the '|' at the sides of the canva
        {
            canva.draw_vertical_line(Point::new(0, 0), canva.get_height(), '|');
            canva.draw_vertical_line(Point::new(1, 0), canva.get_height(), '|');
            canva.draw_vertical_line(Point::new(canva.get_width() - 1, 0), canva.get_height(), '|');
            canva.draw_vertical_line(Point::new(canva.get_width() - 2, 0), canva.get_height(), '|');
        }       

        // This block makes the '=' at the top and the botton of the canva
        {
            canva.draw_horizontal_line(Point::new(0, 0), canva.get_width(), '=');
            canva.draw_horizontal_line(Point::new(0, 1), canva.get_width(), '=');
            canva.draw_horizontal_line(Point::new(0, canva.get_height() - 1), canva.get_width(), '=');
            canva.draw_horizontal_line(Point::new(0, canva.get_height() - 2), canva.get_width(), '=');
        }

        // OBS: due to the lines at the sides of the canva, the working space is X: 2..X-2 and Y: 2..Y-2

        /*  Main state machine.
         *  Calls the apropriated handlers for each state
         *  Breaks the main loop when the state changes to 'Quit'
         */
        match game_state {
            GameState::Menu => handle_menu(&canva, &mut game_state),
            GameState::Turorial => handle_turorial(&canva, &mut game_state),
            GameState::Playing => handle_playing(&canva, &mut game_state, &mut points),
            GameState::Won => handle_won(&canva, &mut game_state),
            GameState::Lost => handle_lost(&canva, &mut game_state),
            GameState::Quit => {break;} //stops the main loop
        }
    }
}


//-------------------------- Get User Input --------------------------//
fn get_input(canva: &Canva) -> char {
    /*  This function places the cursor at the bottom left of the canvas
     *  and then reads and returns the first char that the user inputs
     *  The user can write more than one character, this may cause 
     *  the user text to overwrite things previosly drawn on canvas
     */
    let mut input: String = String::new();
    let mut temp_vec: Vec<char> = Vec::new();
    input.clear();

    canva.draw_horizontal_line(Point::new(3, canva.get_height() - 3), 100, ' ');
    canva.cursor.set_cursor(canva, &Point::new(3, canva.get_height() - 3)).unwrap();

    std::io::stdout().flush().unwrap();

    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().is_empty() {
            input.clear();
        } else {
            break;
        }
    }

    for el in input.trim().chars() {
        temp_vec.push(el);
    }

    temp_vec[0]
}
