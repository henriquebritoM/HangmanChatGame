use crate::{draw_utils::{core::{print_line, print_terminal}, draw_ascii, draw_horizontal_line, draw_vertical_line, text_chars, text_draw, text_with_line_breaker}, extra, get_input, get_random_num, str_to_char, GameState, CANVA};

pub fn handle_menu(array1: &mut [[char; CANVA.yu]; CANVA.xu], state: &mut GameState) {

    let mut input: char;

    menu_draw(array1);
    
    loop {

        input = get_input();

        match input {
            '1' => {*state = GameState::Playing; break},
            '2' => {*state = GameState::Turorial; break},
            '3' => {*state = GameState::Quit; break},
            _ => continue
        }
    }
}

pub fn handle_turorial(array1: &mut [[char; CANVA.yu]; CANVA.xu], state: &mut GameState) {

    let mut input: char;

    turorial_draw(array1);
    
    loop {

        input = get_input();

        match input {
            '1' => {*state = GameState::Menu; break},
            '3' => {*state = GameState::Quit; break},
            _ => continue
        }
    }   
}

// Each indepent element displayed on screen has a fn to help the organization
pub fn handle_playing(array1: &mut [[char; CANVA.yu]; CANVA.xu], state: &mut GameState, points: &mut [i32; 1]) {

    let mut input: char;
    let word: [&str; 2] = extra::words()[get_random_num()];
    let word_chars: Vec<char> = str_to_char(&word[0]);
    let mut word_hidden: Vec<char> = Vec::new();
    let mut used_chars: [char; 26] = ['_'; 26];
    let alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut errors: i8 = 0;
    let mut is_wrong_letter: bool;
    let (start, end, [height1, height2]): (i32, i32, [i32; 2]);

    for _el in &word_chars {
        word_hidden.push('_');
    }

    playing_draw(array1, points);
    
    draw_horizontal_line(2, 20, 22, array1, '=');
    draw_vertical_line(11, 22, 19, array1, '|');
    text_with_line_breaker(CANVA.xu - CANVA.xu / 4 * 3 - 4, 16, CANVA.x / 4 * 3 + 2, array1, word[1]); // This is the hint 
    (start, end, [height1, height2]) = playing_draw_word(array1, [word[0]], &word_hidden);

    print_terminal(array1); // IMPORTANT: do not call this fn in any specific draw fn, it won't bug the program but it will cause double printing, wich is ugly 

     'playing: loop {

        is_wrong_letter = true;  // There is an error unless the program says that there isnt
        input = get_input();
        
        if input == '3' {
            *state = GameState::Quit;
            break 'playing;
        }
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

        text_draw (3, 199, CANVA.y - 3, "left", array1, "Letras usadas: ");
        text_chars(18, CANVA.x, CANVA.y - 3, "left", array1, &used_chars);

        for el in 0..word_chars.len() {

            if word_chars[el] == input {

                word_hidden[el] = input;
                is_wrong_letter = false; // The program is saying that there isnt an error
            }
        }
        if is_wrong_letter {
            errors += 1;
        }

        if errors > 0 && errors < 7 {
            playing_draw_body(errors, array1)
        } 

        for num in height1..height2 {
            draw_horizontal_line(start, end + 2, num, array1,' ');
        }

        playing_draw_word(array1, [word[0]], &word_hidden);
        
        print_terminal(array1);

        if errors == 6 {
            points[0] = 0;
            *state = GameState::Lost;
            break 'playing;
        }

        // The player has discovered the word
        if word_hidden == word_chars {

            points[0] += 1;
            *state = GameState::Won;
            break 'playing;

        }

        // The player has commited too many errors
    }
}

// This plays a little animation
pub fn handle_won(array1: &mut [[char; CANVA.yu]; CANVA.xu], state: &mut GameState) {

    let text: [&str; 5] = [r#"     ____  ___    ____  ___    ____  _______   _______ ____"#,
                           r#"    / __ \/   |  / __ \/   |  / __ )/ ____/ | / / ___// / /"#,
                           r#"   / /_/ / /| | / /_/ / /| | / __  / __/ /  |/ /\__ \/ / / "#,
                           r#"  / ____/ ___ |/ _, _/ ___ |/ /_/ / /___/ /|  /___/ /_/_/  "#,
                           r#" /_/   /_/  |_/_/ |_/_/  |_/_____/_____/_/ |_//____(_|_)   "#,];

    for num in 1..CANVA.x - 2 {
        std::thread::sleep(std::time::Duration::from_millis(15));

        draw_horizontal_line(0, num, 1, array1, '#');
        draw_horizontal_line(0, num, 9, array1, '#');

        draw_vertical_line(1, 10, num, array1, '#');
        draw_vertical_line(2, 9, num - 1, array1, ' ');

        draw_ascii(&text, num - CANVA.x, num, 2, "center", array1);
    
        for n in 1..12 {
            print_line(n, array1)
        };
    } 

    draw_vertical_line(1, 10, 2, array1, '#');
    draw_vertical_line(1, 10, 0, array1, '|');
    draw_vertical_line(1, 10, 1, array1, '|');
    
    for n in 1..12 {
        print_line(n, array1)
    };

    std::thread::sleep(std::time::Duration::from_secs(4));

    *state = GameState::Playing;
}

// This plays a little animation
pub fn handle_lost(array1: &mut [[char; CANVA.yu]; CANVA.xu], state: &mut GameState) {
    
    let text: [&str; 5] = [r#"    _________    __  _________   ____ _    ____________  ____ "#,
                           r#"   / ____/   |  /  |/  / ____/  / __ \ |  / / ____/ __ \/ / / "#,
                           r#"  / / __/ /| | / /|_/ / __/    / / / / | / / __/ / /_/ / / /  "#,
                           r#" / /_/ / ___ |/ /  / / /___   / /_/ /| |/ / /___/ _, _/_/_/   "#,
                           r#" \____/_/  |_/_/  /_/_____/   \____/ |___/_____/_/ |_(_|_)    "#,];

    for num in (1..CANVA.x + 2).rev()  {
        std::thread::sleep(std::time::Duration::from_millis(15));

        draw_horizontal_line(CANVA.x, num, 1, array1, '#');
        draw_horizontal_line(CANVA.x, num, 9, array1, '#');

        draw_vertical_line(1, 10, num, array1, '#');
        draw_vertical_line(2, 9, num + 1, array1, ' ');

        draw_ascii(&text, num, num + CANVA.x, 2, "center", array1);
    
        for n in 1..12 {
            print_line(n, array1)
        };
    } 

    draw_vertical_line(1, 10, CANVA.x - 3, array1, '#');
    draw_vertical_line(1, 10, CANVA.x - 1, array1, '|');
    draw_vertical_line(1, 10, CANVA.x - 2, array1, '|');

    for n in 1..12 {
        print_line(n, array1)
    };

    std::thread::sleep(std::time::Duration::from_secs(4));

    *state = GameState::Menu;

}

// handler to menu
fn menu_draw(array1: &mut [[char; CANVA.yu]; CANVA.xu]) {

    let title: [&str; 6] = [r#"        __                         __         ____                       "#,  // This same block of code appear a good amount of times, but Creating a fn just to print it would be confusing
                            r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _  "#, 
                            r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/  "#, 
                            r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /   "#, 
                            r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/    "#, 
                            r#"            /____/                                                       "#];

    draw_ascii(&title, 2, CANVA.x - 2, 2, "center", array1);

    draw_horizontal_line(2, CANVA.x - 2, 9, array1, '=');
    draw_horizontal_line(2, CANVA.x - 2, 10, array1, '=');

    let one: [&str; 5] = [r#" ,--. "#,   
                          r#"/   | "#,
                          r#"`|  | "#,
                          r#" |  | "#,
                          r#" `--' "#,];

    draw_ascii(&one, 6, 50, 14,"left", array1);
    text_draw (13, 50, 16, "left", array1, "Iniciar o jogo");

    let two: [&str; 5] = [r#" ,---.  "#,
                          r#"'.-.  \ "#,
                          r#" .-' .' "#,
                          r#"/   '-. "#,
                          r#"'-----' "#,];

    draw_ascii(&two, 5, 50, 21,"left", array1);
    text_draw (13, 50, 23, "left", array1, "Ver o tutorial");

    let three: [&str; 5] = [r#" ,----.  "#,
                            r#" '.-.  | "#,
                            r#"   .' <  "#,
                            r#" /'-'  | "#,
                            r#" `----' "#,];

    draw_ascii(&three, 4, 50, 28,"left", array1);
    text_draw (13, 50, 30, "left", array1, "Para sair do jogo a qualquer momento");

    let mut t: usize = 0;
    for n in &extra::words() {
        t += n[0].len() + n[1].len();
    }

    text_with_line_breaker(33, 15, CANVA.x - 35, array1, &format!("Sabia que existem atualmente {} palavras no jogo?! Isso corresponde a exatamentes {} caracteres sendo usados, impressionante, não é mesmo?", &extra::words().len().to_string(), t));
    text_with_line_breaker(32, 21, CANVA.x - 35, array1, "E tudo isso foi feito com o auxílio do chat GPT, então não me responsabilizo por dicas incoerentes ou problemas do tipo!!!");

    text_draw (0,CANVA.x - 3, CANVA.y - 3, "right", array1, "Feito por: Henrique de Brito"); 

    print_terminal(array1);

}

// this section contains the drawers for each handler


fn turorial_draw(array1: &mut [[char; CANVA.yu]; CANVA.xu]) {

    let title: [&str; 6] = [r#"        __                         __         ____                      "#, 
                            r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#, 
                            r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#, 
                            r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#, 
                            r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#, 
                            r#"            /____/                                                      "#];

    draw_ascii(&title, 2, CANVA.x - 2, 2, "center", array1);

    draw_horizontal_line(2, CANVA.x - 2, 9, array1, '=');
    draw_horizontal_line(2, CANVA.x - 2, 10, array1, '=');

    let one: [&str; 5] = [r#" ,--. "#, 
                          r#"/   | "#, 
                          r#"`|  | "#, 
                          r#" |  | "#,
                          r#" `--' "#,];

    draw_ascii(&one, 4, 50, CANVA.y - 9,"left", array1);
    text_draw (12, 50, CANVA.y - 7, "left", array1, "Voltar ao menu");

       
    draw_horizontal_line(2, 20, 22, array1, '=');
    draw_vertical_line(11, 22, 19, array1, '|');

    let sign_left: [&str; 6] =
       [r#"    ██╗       "#,
        r#"   ██╔╝       "#,
        r#"  ██╔╝█████╗  "#,
        r#"  ╚██╗╚════╝  "#,
        r#"   ╚██╗       "#,
        r#"    ╚═╝       "#,];
    
    let _sign_right: [&str; 6] =
        [r#"      ██╗    "#,
         r#"       ╚██╗  "#,
         r#"  █████╗╚██╗ "#,
         r#"  ╚════╝██╔╝ "#,
         r#"      ██╔╝   "#,
         r#"      ╚═╝    "#,];

    let sign_up: [&str; 5] =
       [r#" ████╗  "#,
        r#"██████╗ "#,
        r#"╚═██╔═╝ "#,
        r#"  ██║   "#,
        r#"  ╚═╝   "#];

    let sign_down = 
        [r#"  ╔═╗"#,
         r#"  ██║   "#,
         r#"╔═██╚═╗"#,
         r#"██████╝"#,
         r#" ████╝  "#,];

    draw_ascii(&sign_left, 20, CANVA.x, 13, "left", array1);
    text_draw (34, 50, 15, "left", array1, "Aqui você pode ver suas vidas");
    text_draw (34, 50, 16, "left", array1, "Quanto mais letras errar, mais");
    text_draw (34, 50, 17, "left", array1, "Mais partes do enforcado aparecerão!");
    playing_draw_body(1, array1);

    text_with_line_breaker(CANVA.xu - CANVA.xu / 4 * 3 - 4, 19, CANVA.x / 4 * 3 + 2, array1, "Aqui irá aparecer uma dica útil para a resolução da charada, não esqueça de sempre dar uma olhadinha aqui para entender melhor o desafio. O sistema de quebra de linhas aqui também é muito monstro, adoro essa parte!");
    draw_ascii(&sign_up, CANVA.x - CANVA.x / 4 + 20, CANVA.x, 25, "left", array1);
    draw_vertical_line(11, CANVA.y - 2, CANVA.x / 4 * 3, array1, '|');

    text_draw(2, CANVA.x, CANVA.y - 3, "left", array1, "__________________________");
    text_draw(28, CANVA.x, CANVA.y - 4, "left", array1, "   Aqui irão aparecer TODAS as letras que você usou");
    text_draw(28, CANVA.x, CANVA.y - 3, "left", array1, "<- Repetir letras não consome vida, mas é chato");

    playing_draw_word(array1, ["feijoada"], &vec!['f', 'e', '_', '_', 'o', 'a', '_', '_']);
    draw_ascii(&sign_down, CANVA.x / 2 - 4, CANVA.x, 22, "left", array1);
    text_with_line_breaker(CANVA.xu - CANVA.xu / 4 * 3 - 4, 18, 77, array1, "Aqui fica a sua palavra secreta, as letras serão reveladas na medida em que você as acertar!");

    text_with_line_breaker(CANVA.xu - CANVA.xu / 4 * 3 - 4, 30, CANVA.x / 4 * 3 + 2, array1, "E lembre-se : Sempre que quiser sair do jogo, basta digitar 3!");

    text_draw(0, CANVA.x - CANVA.x / 4 - 1, 12, "right", array1, "Você tem 7 pontos!");
    text_draw(0, CANVA.x - CANVA.x / 4 - 18, 12, "right", array1, "Aqui você pode encontrar sua pontuação ->");

    text_draw(0, CANVA.x - 2, CANVA.y - 3, "right", array1, "Feito por: Henrique Brito");

    print_terminal(array1);
}

fn playing_draw(array1: &mut [[char; CANVA.yu]; CANVA.xu], points: &mut [i32; 1]) {

    let title: [&str; 6] = [r#"        __                         __         ____                      "#, 
                            r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#, 
                            r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#, 
                            r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#, 
                            r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#, 
                            r#"            /____/                                                      "#];

    draw_ascii(&title, 2, CANVA.x - 2, 2, "center", array1);

    draw_horizontal_line(2, CANVA.x - 2, 9, array1, '=');
    draw_horizontal_line(2, CANVA.x - 2, 10, array1, '=');
    draw_vertical_line(11, CANVA.y - 2, CANVA.x / 4 * 3, array1, '|');
    text_draw (CANVA.x / 4 * 3 + 1, CANVA.x - 2, 12, "center", array1, "Fica a Dica");

    text_draw (CANVA.x, CANVA.x - CANVA.x / 4 - 1, 12, "right", array1, &format!("Você tem {} pontos!", &points[0]));
}

fn playing_draw_word(array1: &mut [[char; CANVA.yu]; CANVA.xu], word: [&str; 1], word_hidden: &Vec<char>) -> (i32, i32, [i32; 2]) {

    let word_chars: Vec<char> = str_to_char(word[0]);
    let mut start: i32;
    let start_ref: i32;
    let end: i32;
    let mut size: i32 = 0;

    if (word_chars.len() * 11) >= (CANVA.xu - CANVA.xu / 3 - 2) {
            panic!("A palavra secreta passada é grande demais para a tela!!: {} ", word[0]);    
    }

    for char in word_hidden {
        size += &extra::char_to_ascii(*char).1;
    }

    if (word_chars.len() * 11 / 2 + CANVA.xu / 2) < (CANVA.xu - CANVA.xu / 4 - 1) {  // The word can be drawn in using the absolute middle (X / 2)

        start = CANVA.x / 2 - size / 2;
        end = CANVA.x / 2 + size / 2;
        start_ref = start;
    } else {  // The word needs to be drawn using more space
        start = (CANVA.x - CANVA.x / 4) / 2 - size / 2;
        end = (CANVA.x - CANVA.x / 4) / 2 + size / 2;
        start_ref = start;
    }

    for char in word_hidden {
            
        draw_ascii(&extra::char_to_ascii(*char).0, start, end,CANVA.y - 13,"left", array1);
        start += &extra::char_to_ascii(*char).1;
    }

    (start_ref - 4, end, [CANVA.y - 13, CANVA.y - 5])

}

fn playing_draw_body(num: i8, array1: &mut [[char; CANVA.yu]; CANVA.xu]) {
    
    let head: ([&str; 6], [i32; 2]) =    ([
        r#"      |      "#,
        r#"  ____|____  "#,
        r#" ╱         ╲ "#,
        r#"|   X   X   |"#, // width 13
        r#"|   ┄┄┄┄┄   |"#,
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
    }

}

