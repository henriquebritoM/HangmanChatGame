/*  This file contains the handlers for each game state */
/*  Each handler has autonomy over the canvas and the
 *  game behaviour.
 */

//-------------------------- Crates declaration --------------------------//
use crate::{
    dutils::{str_to_char, Alignment, Canva, Point},
    extra::{self, Word, WORDS},
    get_input, GameState,
};

//-------------------------- Handlers declaration --------------------------//
pub fn handle_menu(canva: &Canva, state: &mut GameState) {
    let mut input: char;

    menu_draw(&canva);

    loop {
        input = get_input(canva);

        match input {
            '1' => {
                *state = GameState::Playing;
                break;
            }
            '2' => {
                *state = GameState::Turorial;
                break;
            }
            '3' => {
                *state = GameState::Quit;
                break;
            }
            _ => continue,
        }
    }
}

pub fn handle_turorial(canva: &Canva, state: &mut GameState) {
    let mut input: char;

    turorial_draw(&canva);

    loop {
        input = get_input(canva);

        match input {
            '1' => {
                *state = GameState::Menu;
                break;
            }
            '3' => {
                *state = GameState::Quit;
                break;
            }
            _ => continue,
        }
    }
}

// Each indepent element displayed on screen has a fn to help the organization
pub fn handle_playing(canva: &Canva, state: &mut GameState, points: &mut u8) {
    let mut input: char;
    let word: Word = extra::get_word();
    let word_chars: Vec<char> = str_to_char(word.get_name());
    let mut word_hidden: Vec<char> = Vec::new();
    let mut used_chars: [char; 26] = ['_'; 26];
    let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut errors: i8 = 0;
    let mut is_wrong_letter: bool;

    for _el in &word_chars {
        word_hidden.push('_');
    }

    playing_draw(&canva, points);
    playing_draw_word(canva, word.get_name(), &word_hidden);

    canva.draw_horizontal_line(Point::new(2, 22), 18, '=');
    canva.draw_vertical_line(Point::new(19, 11),11, '|');
    // prints the 'hint'
    let text_left_corner: Point = Point::new(canva.get_width() / 4 * 3 + 2, 16);
    canva.text_with_line_breaker(
        &text_left_corner,
        usize::try_from(canva.get_width() - canva.get_width() / 4 * 3 - 4).unwrap(),
        
        word.get_description(),
    );

    'playing: loop {

        is_wrong_letter = true; // There is an error unless the program says that there isnt
        input = get_input(canva);

        if input == '3' {
            *state = GameState::Quit;
            break 'playing;
        }
        if !input.is_alphabetic() {
            continue 'playing;
        } // Checks if the input is valid

        for el in used_chars {
            if input == el {
                continue 'playing;
            }
        }

        for el in 0..alphabet.len() {
            if input == alphabet[el] {
                used_chars[el] = input;
                break;
            } else {
                continue;
            }
        }

        canva.text_draw(
            &Point::new(3, canva.get_height() - 4),
            &Alignment::Left,
            "Letras usadas: ",
        );
        {
            //draw individualy each one one the used letter ( '_' if not used)
            let mut point: Point = Point::new(18, canva.get_height() - 4);
            for char in used_chars {
                canva.draw_char(char, &point);
                point.set_x(point.get_x() + 1);
            }
        }

        playing_erase_word(&canva, word.get_name(), &word_hidden);

        for el in 0..word_chars.len() {
            if word_chars[el] == input {
                word_hidden[el] = input;
                is_wrong_letter = false; // The program is saying that there isnt an error
            }
        }

        playing_draw_word(canva, word.get_name(), &word_hidden);

        if is_wrong_letter {
            errors += 1;
        }

        if errors > 0 && errors < 7 {
            playing_draw_body(errors, &canva)
        }

        if errors == 6 {
            *points = 0;
            *state = GameState::Lost;
            break 'playing;
        }

        // The player has discovered the word
        if word_hidden == word_chars {
            *points += 1;
            *state = GameState::Won;
            break 'playing;
        }

        // The player has commited too many errors
    }
}

// This plays a little animation
pub fn handle_won(canva: &Canva, state: &mut GameState) {
    let text: [&str; 5] = [
        r#"     ____  ___    ____  ___    ____  _______   _______ ____"#,
        r#"    / __ \/   |  / __ \/   |  / __ )/ ____/ | / / ___// / /"#,
        r#"   / /_/ / /| | / /_/ / /| | / __  / __/ /  |/ /\__ \/ / / "#,
        r#"  / ____/ ___ |/ _, _/ ___ |/ /_/ / /___/ /|  /___/ /_/_/  "#,
        r#" /_/   /_/  |_/_/ |_/_/  |_/_____/_____/_/ |_//____(_|_)   "#,
    ];

    for num in 1..canva.get_width() - 2 {
        std::thread::sleep(std::time::Duration::from_millis(15));

        canva.draw_horizontal_line(Point::new(0, 1), num, '#');
        canva.draw_horizontal_line(Point::new(0, 9), num, '#');

        canva.draw_vertical_line(Point::new(num, 1), 9, '#');
        canva.draw_vertical_line(Point::new(num - 1, 2), 7, ' ');

        canva.draw_ascii(&text, &Point::new(num - canva.get_width() / 2, 2), &Alignment::Center);
    }

    canva.draw_vertical_line(Point::new(2, 1), 9, '#');
    canva.draw_vertical_line(Point::new(0, 1), 9, '|');
    canva.draw_vertical_line(Point::new(1, 1), 9, '|');

    std::thread::sleep(std::time::Duration::from_secs(4));

    *state = GameState::Playing;
}

// This plays a little animation
pub fn handle_lost(canva: &Canva, state: &mut GameState) {
    let text: [&str; 5] = [
        r#"    _________    __  _________   ____ _    ____________  ____ "#,
        r#"   / ____/   |  /  |/  / ____/  / __ \ |  / / ____/ __ \/ / / "#,
        r#"  / / __/ /| | / /|_/ / __/    / / / / | / / __/ / /_/ / / /  "#,
        r#" / /_/ / ___ |/ /  / / /___   / /_/ /| |/ / /___/ _, _/_/_/   "#,
        r#" \____/_/  |_/_/  /_/_____/   \____/ |___/_____/_/ |_(_|_)    "#,
    ];

    for num in (0..canva.get_width() + 2).rev() {
        std::thread::sleep(std::time::Duration::from_millis(15));

        canva.draw_horizontal_line(Point::new(canva.get_width(), 1), num, '#');
        canva.draw_horizontal_line(Point::new(canva.get_width(), 8), num, '#');

        canva.draw_vertical_line(Point::new(num + 2, 1), 9, '#');
        canva.draw_vertical_line(Point::new(num + 3, 2), 7, ' ');

        canva.draw_ascii(&text, &Point::new(num + canva.get_width() / 2, 3), &Alignment::Center);
    }

    canva.draw_vertical_line(Point::new(canva.get_width() - 1, 1), 9, '|');
    canva.draw_vertical_line(Point::new(canva.get_width() - 2, 1), 9, '|');
    canva.draw_vertical_line(Point::new(canva.get_width() - 3, 1), 9, '#');

    std::thread::sleep(std::time::Duration::from_secs(4));

    *state = GameState::Menu;
}

// handler to menu
fn menu_draw(canva: &Canva) {
    let title: [&str; 6] = [
        r#"        __                         __         ____                       "#, // This same block of code appear a good amount of times, but Creating a fn just to print it would be confusing
        r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _  "#,
        r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/  "#,
        r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /   "#,
        r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/    "#,
        r#"            /____/                                                       "#,
    ];

    canva.draw_ascii(&title, &Point::new(canva.get_width() / 2, 2), &Alignment::Center);

    canva.draw_horizontal_line(Point::new(2, 9), canva.get_width() - 4, '=');
    canva.draw_horizontal_line(Point::new(2, 10), canva.get_width() - 4, '=');

    let one: [&str; 5] = [
        r#" ,--. "#,
        r#"/   | "#,
        r#"`|  | "#,
        r#" |  | "#,
        r#" `--' "#,
    ];

    canva.draw_ascii(&one, &Point::new(6, 14), &Alignment::Left);
    canva.text_draw(&Point::new(13, 16), &Alignment::Left, "Iniciar o jogo");

    let two: [&str; 5] = [
        r#" ,---.  "#,
        r#"'.-.  \ "#,
        r#" .-' .' "#,
        r#"/   '-. "#,
        r#"'-----' "#,
    ];

    canva.draw_ascii(&two, &Point::new(5, 21), &Alignment::Left);
    canva.text_draw(&Point::new(13, 23), &Alignment::Left, "Ver o tutorial");

    let three: [&str; 5] = [
        r#" ,----.  "#,
        r#" '.-.  | "#,
        r#"   .' <  "#,
        r#" /'-'  | "#,
        r#" `----' "#,
    ];

    canva.draw_ascii(&three, &Point::new(4, 28), &Alignment::Left);
    canva.text_draw(
        &Point::new(13, 30),
        &Alignment::Left,
        "Para sair do jogo a qualquer momento",
    );

    let mut total_characters: usize = 0;
    for n in WORDS {
        total_characters += n[0].len() + n[1].len();
    }

    canva.text_with_line_breaker(&Point::new(canva.get_width() - 35, 15),33, &format!("Sabia que existem atualmente {} palavras no jogo?! \
                                                                                                        Isso corresponde a exatamentes {} caracteres sendo usados, \
                                                                                                        impressionante, não é mesmo?", WORDS.len().to_string(), total_characters));
    canva.text_with_line_breaker(&Point::new(canva.get_width() - 35, 21),32,  "E tudo isso foi feito com o auxílio do chat GPT, então não me \
                                                                                                        responsabilizo por dicas incoerentes ou problemas do tipo!!!");

    canva.text_draw(
        &Point::new(canva.get_width() - 3, canva.get_height() - 3),
        &Alignment::Right,
        "Feito por: Henrique de Brito",
    );
}

// this section contains the drawers for each handler

fn turorial_draw(canva: &Canva) {
    let title: [&str; 6] = [
        r#"        __                         __         ____                      "#,
        r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#,
        r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#,
        r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#,
        r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#,
        r#"            /____/                                                      "#,
    ];

    canva.draw_ascii(&title, &Point::new(canva.get_width() / 2, 2), &Alignment::Center);

    canva.draw_horizontal_line(Point::new(2, 9), canva.get_width() - 4, '=');
    canva.draw_horizontal_line(Point::new(2, 9), canva.get_width() - 4, '=');

    let one: [&str; 5] = [
        r#" ,--. "#,
        r#"/   | "#,
        r#"`|  | "#,
        r#" |  | "#,
        r#" `--' "#,
    ];

    canva.draw_ascii(&one, &Point::new(4, canva.get_height() - 9), &Alignment::Left);
    canva.text_draw(
        &Point::new(12, canva.get_height() - 7),
        &Alignment::Left,
        "Voltar ao menu",
    );

    canva.draw_horizontal_line(Point::new(2, 22), 18, '=');
    canva.draw_vertical_line(Point::new(19, 10), 12, '|');

    let sign_left: [&str; 6] = [
        r#"    ██╗       "#,
        r#"   ██╔╝       "#,
        r#"  ██╔╝█████╗  "#,
        r#"  ╚██╗╚════╝  "#,
        r#"   ╚██╗       "#,
        r#"    ╚═╝       "#,
    ];

    let _sign_right: [&str; 6] = [
        r#"      ██╗    "#,
        r#"       ╚██╗  "#,
        r#"  █████╗╚██╗ "#,
        r#"  ╚════╝██╔╝ "#,
        r#"      ██╔╝   "#,
        r#"      ╚═╝    "#,
    ];

    let sign_up: [&str; 5] = [
        r#" ████╗  "#,
        r#"██████╗ "#,
        r#"╚═██╔═╝ "#,
        r#"  ██║   "#,
        r#"  ╚═╝   "#,
    ];

    let sign_down = [
        r#"  ╔═╗"#,
        r#"  ██║   "#,
        r#"╔═██╚═╗"#,
        r#"██████╝"#,
        r#" ████╝  "#,
    ];

    canva.draw_ascii(&sign_left, &Point::new(20, 13), &Alignment::Left);
    canva.text_draw(
        &Point::new(34, 15),
        &Alignment::Left,
        "Aqui você pode ver suas vidas",
    );
    canva.text_draw(
        &Point::new(34, 16),
        &Alignment::Left,
        "Quanto mais letras errar, mais",
    );
    canva.text_draw(
        &Point::new(34, 17),
        &Alignment::Left,
        "Mais partes do enforcado aparecerão!",
    );
    playing_draw_body(1, &canva);

    canva.text_with_line_breaker(&Point::new(canva.get_width() / 4 * 3 + 2, 19),(canva.get_width() - canva.get_width() / 4 * 3 - 4).try_into().unwrap(),
                                "Aqui irá aparecer uma dica útil para a resolução da charada, não esqueça de sempre dar uma olhadinha aqui para \
                                      entender melhor o desafio. O sistema de quebra de linhas aqui também é muito monstro, adoro essa parte!");
    canva.draw_ascii(
        &sign_up,
        &Point::new(canva.get_width() - canva.get_width() / 4 + 20, 25), &Alignment::Left
    );
    canva.draw_vertical_line(Point::new(canva.get_width() / 4 * 3, 10), canva.get_height() - 12, '|');

    canva.text_draw(&Point::new(3, canva.get_height() - 4), &Alignment::Left,"Letras usadas: __________________________");

    canva.text_draw(
        &Point::new(43, canva.get_height() - 5),
        &Alignment::Left,
        "   Aqui irão aparecer TODAS as letras que você usou",
    );
    canva.text_draw(
        &Point::new(43, canva.get_height() - 5),
        &Alignment::Left,
        "<- Repetir letras não consumira vidas!",
    );

    canva.text_draw(&Point::new(5, canva.get_height() - 3), &Alignment::Left, "<- Seu cursor aparecerá aqui!");

    playing_draw_word(
        &canva,
        "feijoada",
        &vec!['f', 'e', '_', '_', 'o', 'a', '_', '_'],
    );
    canva.draw_ascii(&sign_down, &Point::new(canva.get_width() / 2 - 4, 22), &Alignment::Left);
    canva.text_with_line_breaker(&Point::new(77, 18),((canva.get_width() - canva.get_width() / 4 * 3 - 4)).try_into().unwrap(),  
    "Aqui fica a sua palavra secreta, as letras serão reveladas na medida em que você as acertar!");

    canva.text_with_line_breaker(
        &Point::new(canva.get_width() / 4 * 3 + 2, 30),
        (canva.get_width() - canva.get_width() / 4 * 3 - 4).try_into().unwrap(),
        
        "E lembre-se : Sempre que quiser sair do jogo, basta digitar 3!",
    );

    canva.text_draw(
        &Point::new(canva.get_width() - canva.get_width() / 4 - 1, 12),
        &Alignment::Right,
        "Você tem 7 pontos!",
    );
    canva.text_draw(
        &Point::new(canva.get_width() - canva.get_width() / 4 - 18, 12),
        &Alignment::Right,
        "Aqui você pode encontrar sua pontuação ->",
    );

    canva.text_draw(
        &Point::new(canva.get_width() - 2, canva.get_height() - 3),
        &Alignment::Right,
        "Feito por: Henrique Brito",
    );

}

fn playing_draw(canva: &Canva, points: &mut u8) {
    let title: [&str; 6] = [
        r#"        __                         __         ____                      "#,
        r#"       / /___  ____ _____     ____/ /___ _   / __/___  ______________ _ "#,
        r#"  __  / / __ \/ __ `/ __ \   / __  / __ `/  / /_/ __ \/ ___/ ___/ __ `/ "#,
        r#" / /_/ / /_/ / /_/ / /_/ /  / /_/ / /_/ /  / __/ /_/ / /  / /__/ /_/ /  "#,
        r#" \____/\____/\__, /\____/   \__,_/\__,_/  /_/  \____/_/   \___/\__,_/   "#,
        r#"            /____/                                                      "#,
    ];

    canva.draw_ascii(&title, &Point::new(canva.get_width() / 2,  2), &Alignment::Center);

    canva.draw_horizontal_line(Point::new(2, 9), canva.get_width() - 4, '=');
    canva.draw_horizontal_line(Point::new(2, 10), canva.get_width() - 4, '=');
    canva.draw_vertical_line(Point::new(canva.get_width() / 4 * 3, 11), canva.get_height() - 13, '|');
    
    canva.text_draw(&Point::new(canva.get_width() / 4 * 3 + 2, 15), &Alignment::Left, "Fica a Dica");
    
    canva.text_cdraw(
        &Point::new(canva.get_width() / 4 - 1, 12),
        canva.get_width() - canva.get_width() / 4 - 1,
        &format!("Você tem {} pontos!", points),
    );
}

fn playing_draw_word(canva: &Canva, word: &str, word_hidden: &Vec<char>) {

    let mut top_left_point: Point = get_word_params(canva, word, word_hidden).0;

    for char in word_hidden {
        canva.draw_ascii(
            &extra::char_to_ascii(*char).0,
            &top_left_point, &Alignment::Left
        );
        top_left_point.set_x(top_left_point.get_x() + &extra::char_to_ascii(*char).1);
    }
}

fn playing_erase_word(canva: &Canva, word: &str, word_hidden: &Vec<char>) {

    let (top_left_point, lenght, rows) = get_word_params(canva, word, word_hidden);

    canva.text_draw(&Point::new(5, 6), &Alignment::Left, &format!("lenght is {:?}", lenght));

    for num in 0..rows {
        println!("erasing line");
        canva.draw_horizontal_line(Point::new(top_left_point.get_x(), top_left_point.get_y() + num), lenght,' ');
    }
    
}

fn get_word_params(canva: &Canva, word: &str, word_hidden: &Vec<char>) -> (Point, i16, i16) {
    let word_chars: Vec<char> = str_to_char(word);
    let top_left_corner: Point;
    let mut lenght: i16 = 0;
    let rows: i16 = 8;

    if i16::try_from(word_chars.len() * 11).unwrap() >= canva.get_width() - canva.get_width() / 3 - 2 {
        panic!(
            "A palavra secreta passada é grande demais para a tela!!: {} ",
            word
        );
    }

    //gets the lenght of the chars that will be displayed ('_' included)
    for char in word_hidden {  
        lenght += &extra::char_to_ascii(*char).1;
    }

    //sets where the left point will be
    //
    // The word can be drawn in using the absolute middle (X / 2)
    if (word_chars.len() * 11 / 2 + usize::try_from(canva.get_width()).unwrap() / 2)
        < usize::try_from(canva.get_width() - canva.get_width() / 4 - 1).unwrap()
    {   
        top_left_corner = Point::new(canva.get_width() / 2 - lenght / 2, canva.get_height() - 13);
    }
    // The word needs to be shifted some spaces to the left
    else {
        top_left_corner = Point::new((canva.get_width() - canva.get_width() / 4) / 2 - lenght / 2, canva.get_height() - 13);
    }

    (top_left_corner, lenght, rows)

}

fn playing_draw_body(num: i8, canva: &Canva) {
    let head: ([&str; 6], [i16; 2]) = (
        [
            r#"      |      "#,
            r#"  ____|____  "#,
            r#" ╱         ╲ "#,
            r#"|   X   X   |"#, // width 13
            r#"|   ┄┄┄┄┄   |"#,
            r#" ╲_________╱ "#,
        ],
        [0, 0],
    );

    let body: ([&str; 2], [i16; 2]) = ([r#"|"#, r#"|"#], [6, 6]);

    let left_arm: ([&str; 2], [i16; 2]) = ([r#" ╱"#, r#"╱ "#], [4, 6]);

    let right_arm: ([&str; 2], [i16; 2]) = ([r#"╲"#, r#" ╲"#], [7, 6]);

    let left_leg: ([&str; 2], [i16; 2]) = ([r#" ╱"#, r#"╱ "#], [4, 8]);

    let right_leg: ([&str; 2], [i16; 2]) = ([r#"╲"#, r#" ╲"#], [7, 8]);

    match num {
        1 => canva.draw_ascii(&head.0, &Point::new(4 + head.1[0], 11 + head.1[1]), &Alignment::Left),
        2 => canva.draw_ascii(&body.0, &Point::new(4 + body.1[0], 11 + body.1[1]), &Alignment::Left),
        3 => canva.draw_ascii(
            &left_arm.0,
            &Point::new(4 + left_arm.1[0], 11 + left_arm.1[1]), &Alignment::Left
        ),
        4 => canva.draw_ascii(
            &right_arm.0,
            &Point::new(4 + right_arm.1[0], 11 + right_arm.1[1]), &Alignment::Left
        ),
        5 => canva.draw_ascii(
            &left_leg.0,
            &Point::new(4 + left_leg.1[0], 11 + left_leg.1[1]), &Alignment::Left
        ),
        6 => canva.draw_ascii(
            &right_leg.0,
            &Point::new(4 + right_leg.1[0], 11 + right_leg.1[1]), &Alignment::Left
        ),
        _ => panic!("A number too big was passed when requiring a body drawning!2"),
    }
}
