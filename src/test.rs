
use std::io::Write;
use rand::Rng;
mod extra;

const X: i32 = 200;
const Y: i32 = 40;
const XU: usize = 200;
const YU: usize = 40;

// XU and YU are meant to be the same values, but for convinience reasons they are split in i32 and usize
fn main() {

    let mut array: [[char; YU]; XU] = [['a'; YU]; XU];
    let mut game_state: GameState = GameState::Menu;
    let mut points: [i32; 1] = [0];
    
    loop { //game main loop 

        // let area_dica: usize = X - X / 4;
        match game_state {
            GameState::Menu => set_to_blank(&mut array),
            GameState::Turorial => set_to_blank(&mut array),
            GameState::Playing => set_to_blank(&mut array),
            GameState::Won => {},
            GameState::Lost => set_to_blank(&mut array),
            GameState::Quit => {break;}, //stops the main loop
        }
       

        // This block makes the '|' at the sides of the canva
        striked_line_vertical(0, Y, 0, &mut array, '|');
        striked_line_vertical(0, Y, 1, &mut array, '|');
        striked_line_vertical(0, Y, X - 1, &mut array, '|');
        striked_line_vertical(0, Y, X - 2, &mut array, '|');

        // And this one makes the '=' at the top and the botton of the space
        striked_line_horizontal(0, X, 1, &mut array, '=');
        striked_line_horizontal(0, X, Y - 2, &mut array, '=');
        striked_line_horizontal(0, X, 0, &mut array, '=');
        striked_line_horizontal(0, X, Y - 1, &mut array, '=');

        // OBS: due to the lines at the sides of the canva, the working space is X: 2..X-2 and Y: 2..Y-2

        match game_state {
            GameState::Menu => handle_menu(&mut array, &mut game_state, &points),
            GameState::Turorial => handle_turorial(&mut array, &mut game_state),
            GameState::Playing => handle_playing(&mut array, &mut game_state, &mut points),
            GameState::Won => handle_won(&mut array, &mut game_state),
            GameState::Lost => handle_lost(),
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

// This function sets all characters in the canvas to an UTF8 blank char, otherwise all the spaces would be filled with 'a's. The blank char used: '⠀'
fn set_to_blank (array1: &mut [[char; YU]; XU]) {

    let mut height: usize = 0;

    while height < array1[0].len() {

        for num in 0..array1.len() {
            
            array1[num][height] = ' ';
        }

    height += 1;

    }

}

// this function places a said char in every space between a given range, only works in horizontal lines, one at a time
fn striked_line_horizontal (start: i32, end: i32, height: i32, array1: &mut [[char; YU]; XU], char: char) {

    let valid_start: usize;
    let valid_end: usize;

    if start > X || end < 0 {
        return;
    } 
    if height < 0 || height > Y {
        return;
    } 

    if start < 0 {
        valid_start = 0;
    } else {
        valid_start = usize::try_from(start).unwrap();
    }
    if end > X {
        valid_end = XU - 1;
    } else {
        valid_end = usize::try_from(end).unwrap()
    }

    for el in valid_start..valid_end {

        array1[el][usize::try_from(height).unwrap()] = char;
    }
}

// todo
// this function places a said char in every space between a given range, only works in vertical lines, one at a time
fn striked_line_vertical (start: i32, end: i32, xpos: i32, array1: &mut [[char; YU]; XU], char: char) {

    let valid_start: usize;
    let valid_end: usize;

    if start > Y || end < 0 {
        return;
    } 
    if xpos < 0 || xpos > X {
        return;
    }


    if start < 0 {
        valid_start = 0;
    } else {
        valid_start = usize::try_from(start).unwrap();
    }
    if end > Y {
        valid_end = YU - 1;
    } else {
        valid_end = usize::try_from(end).unwrap()
    }

    for el in valid_start..valid_end {

        array1[usize::try_from(xpos).unwrap()][el] = char;
    }
}

// This is the main printing function, it cleans the terminal and then prints every element
fn print_terminal (array1: &[[char; YU]; XU]) {

    let mut height: usize = 0;
    print!("\n");

    print!("{esc}c", esc = 27 as char);  // This line doesen't  work very well

    while height < array1[0].len() {

        for num in 0..array1.len() {
            
            print!("{}", array1[num][height]);

        }
    
    print!("\n");

    height += 1;

    }
}

fn print_line (height: usize, array1: &[[char; YU]; XU]) {

    for num in 0..array1.len() {
            
            print!("{}", array1[num][height]);

        }
}

// The start/end variables must be in a valid space, do not count the structural lines, you don't want to overwrite them
fn text_draw (start: i32, end: i32, height: i32, position: &str, array1: &mut [[char; YU]; XU], text: &str) {

    let mut string_start: i32;
    let string_chars: Vec<char> = str_to_char(text);

    if height >= Y {panic!("A height bigger than allowed was passed into text() !")}

    // The word is printed from left to right, with a counter to keep track of the last position. The starting position is defined in the block below
    match position {
        "left" => string_start = start,
 
        "right" => string_start = end - i32::try_from(text.len()).unwrap(),

        "center" => string_start = (start + end) / 2 - (i32::try_from(text.len()).unwrap() / 2),

        &_ => panic!("you put a invalid position in the text function!")
    }

    for character in string_chars {

        if string_start >= 0 {

            array1[usize::try_from(string_start).unwrap()][usize::try_from(height).unwrap()] = character;
        }

        string_start += 1
    }

}

// The start/end variables must be in a valid space, do not count the structural lines, you don't want to overwrite them
// This does the same thing as fn text(), but uses a array of chars instead of an &str
fn text_chars (start: i32, end: i32, height: i32, position: &str, array1: &mut [[char; YU]; XU], text: &[char]) {

    let mut string_start: i32;
    let string_chars: &[char] = text;

    if height >= Y {panic!("A height bigger than allowed was passed into text() !")}

    // The word is printed from left to right, with a counter to keep track of the last position. The starting position is defined in the block below
    match position {
        "left" => string_start = start,
 
        "right" => string_start = end - i32::try_from(text.len()).unwrap(),

        "center" => string_start = (start + end) / 2 - (i32::try_from(text.len()).unwrap() / 2),

        &_ => panic!("you put a invalid position in the text function!")
    }

    for character in string_chars {

        if string_start >= 0 {

            array1[usize::try_from(string_start).unwrap()][usize::try_from(height).unwrap()] = character.clone();
        }

        string_start += 1
    }

}

fn str_to_char (string: &str) -> Vec<char> {

    let mut char_vec = Vec::new();

    for character in string.chars() {
        char_vec.push(character)
    }

    char_vec

}

// Change: draw_ascii now takes a i32 as start value, this allow the function to be used with animations starting from outside the printing area
// Observation: sometimes you may see 199 being passed as end, this in just an arbitrary value, because most uses this isn't used, but 199 is also the biggest end value possible, so is the best arbitrary value to be used
fn draw_ascii (ascii: &[&str], start: i32, end: i32, mut height: i32, position: &str, array1: &mut [[char; 40]; 200]) {

    let mut heigth_counter: i32 = height;

    for el in ascii {

        if height >= 0 || height <= Y - 1 {
            text_draw (start, end, heigth_counter, position, array1, el);
        }

        heigth_counter += 1;

    }
}

fn text_with_line_breaker (max_len: usize, mut line_height: i32, start: i32, array1: &mut [[char; 40]; 200], text: &str) {

    let word_chars: Vec<char> = str_to_char(text);
    let mut pos: usize = 0;
    let mut temp_str: String = String::new();
    let end: i32 = start + i32::try_from(max_len).unwrap();

    loop {

        if (word_chars.len() - pos) <= max_len {

            for i in pos..word_chars.len() {
                temp_str.push(word_chars[i])
            }

            text_draw (start, end, line_height, "left", array1, &temp_str);
            break;
        } 
        else {
            
            for k in (pos..max_len + pos).rev() {
                

                if word_chars[k] == ' ' {

                    for e in pos..k {
                        temp_str.push(word_chars[e])
                    }

                    text_draw(start, end, line_height, "left", array1, &temp_str);
                    pos = k + 1; // This skips the ' ' char
                    line_height += 1;
                    temp_str.clear();
                    break;
                }
            }
        }
    }
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

fn get_random_num() -> usize {

    let mut number = rand::thread_rng();

    number.gen_range(0..extra::words().len());

    6

}

fn handle_menu(array1: &mut [[char; YU]; XU], state: &mut GameState, points: &[i32; 1]) {

    let mut input: char;

    menu_draw(array1, points);
    
    loop {

        input = get_input();

        match input {
            '1' => {*state = GameState::Playing; break},
            '2' => {*state = GameState::Turorial; break},
            'c' => {*state = GameState::Quit; break},
            _ => continue
        }
    }
}

fn handle_turorial(array1: &mut [[char; YU]; XU], state: &mut GameState) {

    let mut input: char;

    turorial_draw(array1);
    
    loop {

        input = get_input();

        match input {
            '1' => {*state = GameState::Menu; break},
            'c' => {*state = GameState::Quit; break},
            _ => continue
        }
    }

    
}

// Each indepent element displayed on screen has a fn to help the organization
fn handle_playing(array1: &mut [[char; YU]; XU], state: &mut GameState, points: &mut [i32; 1]) {

    let mut input: char;
    let word: [&str; 2] = extra::words()[get_random_num()];
    let word_chars: Vec<char> = str_to_char(&word[0]);
    let mut word_hidden: Vec<char> = Vec::new();
    let mut used_chars: [char; 26] = ['_'; 26];
    let alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut errors: i8 = 0;
    let mut error_counter: bool;
    let (start, end, [height1, height2]): (i32, i32, [i32; 2]);

    for _el in &word_chars {
        word_hidden.push('_');
    }

    playing_draw(array1);
    
    striked_line_horizontal(2, 20, 22, array1, '=');
    striked_line_vertical(11, 22, 19, array1, '|');
    text_with_line_breaker(XU - XU / 4 * 3 - 4, 16, X / 4 * 3 + 2, array1, word[1]); // This is the hint 
    (start, end, [height1, height2]) = playing_draw_word(array1, [word[0]], &word_hidden);

    print_terminal(array1); // IMPORTANT: do not call this fn in any specific draw fn, it won't bug the program but it will cause double printing, wich is ugly 

     'playing: loop {

        error_counter = true;  // There is an error unless the program says that there isnt
        input = get_input();

        if !input.is_alphabetic() {continue 'playing} // Checks if the input is valid

        for el in used_chars {
            if input == el {
                continue 'playing;
            }
        }

        for el in 0..alphabet.len() {

            if input == alphabet[el] {

                used_chars[el] = input;
                break;

            } else {continue}  
        }

        text_draw (3, 199, Y - 3, "left", array1, "Letras usadas: ");
        text_chars(18, X, Y - 3, "left", array1, &used_chars);

        for el in 0..word_chars.len() {

            if word_chars[el] == input {

                word_hidden[el] = input;
                error_counter = false; // The program is saying that there isnt an error
            }
        }
        if error_counter {
            errors += 1;
        }

        if errors > 6 {panic!("A number too big was passed when requiring a body drawning!1")};

        if errors > 0 {
            playing_draw_body(errors, array1)
        }  

        for num in height1..height2 {
            striked_line_horizontal(start, end + 2, num, array1,' ');
        }

        playing_draw_word(array1, [word[0]], &word_hidden);
        
        print_terminal(array1);

        // The player has discovered the word
        if word_hidden == word_chars {
            let mut size: i32 = 0;
            let mut start: i32 = (X - X / 4) / 2;
            let message: &str = "parabens";

            for char in str_to_char(message) {
                size += &extra::char_to_ascii(char).1;
            }

            start -= size / 2;

            for char in str_to_char(message) {
                draw_ascii(&extra::char_to_ascii(char).0, start, 199, 16, "left", array1);
                start += &extra::char_to_ascii(char).1;
            }

            print_terminal(array1);

            points[0] += 1;
            *state = GameState::Won;
            break 'playing;

        }

        // The player has commited too many errors
    }
}

// This plays a little animation
fn handle_won(array1: &mut [[char; YU]; XU], state: &mut GameState) {

    let text: [&str; 5] = [r#"     ____  ___    ____  ___    ____  _______   _______ ____"#,
                           r#"    / __ \/   |  / __ \/   |  / __ )/ ____/ | / / ___// / /"#,
                           r#"   / /_/ / /| | / /_/ / /| | / __  / __/ /  |/ /\__ \/ / / "#,
                           r#"  / ____/ ___ |/ _, _/ ___ |/ /_/ / /___/ /|  /___/ /_/_/  "#,
                           r#" /_/   /_/  |_/_/ |_/_/  |_/_____/_____/_/ |_//____(_|_)   "#,];

    for num in 1..X - 2 {
        std::thread::sleep(std::time::Duration::from_millis(30));

        striked_line_horizontal(0, num, 1, array1, '#');
        striked_line_horizontal(0, num, 9, array1, '#');

        striked_line_vertical(1, 10, num, array1, '#');
        striked_line_vertical(2, 9, num - 1, array1, ' ');

        draw_ascii(&text, num - X, num, 2, "center", array1);
    
        print_terminal(array1);
    } 

    striked_line_vertical(1, 10, 2, array1, '#');
    striked_line_vertical(1, 10, 0, array1, '|');
    striked_line_vertical(1, 10, 1, array1, '|');
    print_terminal(array1);

    std::thread::sleep(std::time::Duration::from_secs(4));

    *state = GameState::Menu;
}

fn handle_lost() {}

fn menu_draw(array1: &mut [[char; YU]; XU], points: &[i32; 1]) {

    let title: [&str; 6] = [r#"        __                         __         ____                      "#,  // This same block of code appear a good amount of times, but Creating a fn just to print it would be confusing
                            r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#, 
                            r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#, 
                            r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#, 
                            r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#, 
                            r#"            /____/                                                      "#];

    draw_ascii(&title, 2, X - 2, 2, "center", array1);

    striked_line_horizontal(2, X - 2, 9, array1, '=');
    striked_line_horizontal(2, X - 2, 10, array1, '=');

    let one: [&str; 5] = [r#" ,--. "#,
                          r#"/   | "#,
                          r#"`|  | "#,
                          r#" |  | "#,
                          r#" `--' "#,];

    draw_ascii(&one, 4, 50, 18,"left", array1);
    text_draw (12, 50, 20, "left", array1, "Iniciar o jogo");

    let two: [&str; 5] = [r#" ,---.  "#,
                          r#"'.-.  \ "#,
                          r#" .-' .' "#,
                          r#"/   '-. "#,
                          r#"'-----' "#,];

    draw_ascii(&two, 4, 50, 27,"left", array1);
    text_draw (12, 50, 29, "left", array1, "Ver o tutorial");

    let message: String = "Sabia que existem atualmente ".to_string() + &extra::words().len().to_string() + " palavras no jogo o que e algo muito legal e divertido?!";
    println!("{}", message);

    text_with_line_breaker(20, 15, X - 23, array1, &message);

    //Draw points counter
    let msg: String = "Você tem ".to_string() + &points[0].to_string() + " pontos!";
    text_draw (X - X / 6, X - 10, 12, "left", array1, &msg);

    text_draw (0,X - 3, Y - 4, "right", array1, "Feito por: Henrique de Brito"); 

    print_terminal(array1);

}

fn turorial_draw(array1: &mut [[char; YU]; XU]) {

    let title: [&str; 6] = [r#"        __                         __         ____                      "#, 
                            r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#, 
                            r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#, 
                            r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#, 
                            r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#, 
                            r#"            /____/                                                      "#];

    draw_ascii(&title, 2, X - 2, 2, "center", array1);

    striked_line_horizontal(2, X - 2, 9, array1, '=');
    striked_line_horizontal(2, X - 2, 10, array1, '=');

    let one: [&str; 5] = [r#" ,--. "#, // line Y - 7
                          r#"/   | "#, // Line Y - 6
                          r#"`|  | "#, // Line Y - 5 -- Mental note because this is confusing
                          r#" |  | "#,
                          r#" `--' "#,];

    draw_ascii(&one, 4, 50, Y - 7,"left", array1);
    text_draw (12, 50, Y - 5, "left", array1, "Voltar ao menu");

    print_terminal(array1);
}

fn playing_draw(array1: &mut [[char; YU]; XU]) {

    let title: [&str; 6] = [r#"        __                         __         ____                      "#, 
                            r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#, 
                            r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#, 
                            r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#, 
                            r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#, 
                            r#"            /____/                                                      "#];

    draw_ascii(&title, 2, X - 2, 2, "center", array1);

    striked_line_horizontal(2, X - 2, 9, array1, '=');
    striked_line_horizontal(2, X - 2, 10, array1, '=');
    striked_line_vertical(11, Y - 2, X / 4 * 3, array1, '|');
    text_draw (X / 4 * 3 + 1, X - 2, 12, "center", array1, "Fica a Dica");
}

fn playing_draw_word(array1: &mut [[char; YU]; XU], word: [&str; 1], word_hidden: &Vec<char>) -> (i32, i32, [i32; 2]) {

    let word_chars: Vec<char> = str_to_char(word[0]);
    let mut start: i32;
    let start_ref: i32;
    let mut end: i32;
    let mut size: i32 = 0;

    if (word_chars.len() * 11) >= (XU - XU / 3 - 2) {
            panic!("A palavra secreta passada é grande demais para a tela!!: {} ", word[0]);    
    }

    for char in word_hidden {
        size += &extra::char_to_ascii(*char).1;
    }

    if (word_chars.len() * 11 / 2 + XU / 2) < (XU - XU / 4) {  // The word can be drawn in using the absolute middle (X / 2)

        start = X / 2 - size / 2;
        end = X / 2 + size / 2;
        start_ref = start;
    } else {  // The word needs to be drawn using more space
        start = (X - X / 4) / 2 - size / 2;
        end = (X - X / 4) / 2 + size / 2;
        start_ref = start;
    }

    for char in word_hidden {
            
        draw_ascii(&extra::char_to_ascii(*char).0, start, end,Y - 13,"left", array1);
        start += &extra::char_to_ascii(*char).1;
    }

    (start_ref, end, [Y - 13, Y - 6])

}

fn playing_draw_body(num: i8, array1: &mut [[char; YU]; XU]) {
    
    let head: ([&str; 6], [i32; 2]) =    ([
        r#"      |      "#,
        r#"  ____|____  "#,
        r#" ╱         ╲ "#,
        r#"⎹   X   X   ⎸"#, // width 13
        r#"⎹   ┄┄┄┄┄   ⎸"#,
        r#" ╲_________╱ "#], [0, 0]);

    let body: ([&str; 2], [i32; 2]) =    ([
            r#"|"#,
            r#"|"#], [6, 6]);

    let left_arm: ([&str; 2], [i32; 2]) =    ([
            r#" ╱"#,
            r#"╱ "#], [4, 6]);

    let right_arm: ([&str; 2], [i32; 2]) = ([
            r#"╲"#,
            r#" ╲"#], [7, 6]);

    let left_leg: ([&str; 2], [i32; 2]) = ([
            r#" ╱"#,
            r#"╱ "#], [4, 8]);

    let right_leg: ([&str; 2], [i32; 2]) = ([
            r#"╲"#,
            r#" ╲"#], [7, 8]);
 
    match num {
        1 => draw_ascii(&head.0, 4 + head.1[0], 25, 11 + head.1[1], "left", array1),
        2 => draw_ascii(&body.0, 4 + body.1[0], 25, 11 + body.1[1], "left", array1),
        3 => draw_ascii(&left_arm.0, 4 + left_arm.1[0], 25, 11 + left_arm.1[1], "left", array1),
        4 => draw_ascii(&right_arm.0, 4 + right_arm.1[0], 25,11 + right_arm.1[1], "left", array1),
        5 => draw_ascii(&left_leg.0, 4 + left_leg.1[0], 25, 11 + left_leg.1[1], "left", array1),
        6 => draw_ascii(&right_leg.0, 4 + right_leg.1[0], 25,11 + right_leg.1[1], "left", array1),
        _ => panic!("A number too big was passed when requiring a body drawning!2"),
        //=> panic!("A number too big was passed when requiring a body drawning!2"),
    }

}

fn won_draw() {}

fn lost_draw() {}
