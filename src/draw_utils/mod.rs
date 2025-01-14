use core::draw_char;
use core::Point;
use crate::str_to_char;
use crate::CANVA;

pub mod core;

// this function places a said char in every space between a given range, only works in horizontal lines, one at a time
pub fn draw_horizontal_line (start: i32, end: i32, height: i32, array1: &mut [[char; CANVA.yu]; CANVA.xu], character: char) {

    let valid_start: usize;
    let valid_end: usize;
    let mut point: Point;

    if start > CANVA.x || end < 0 {
        return;
    } 
    if height < 0 || height >= CANVA.y {
        return;
    } 

    if start < 0 {
        valid_start = 0;} 
    else {
        valid_start = usize::try_from(start).unwrap();
    }

    if end > CANVA.x {
        valid_end = CANVA.xu;} 
    else {
        valid_end = usize::try_from(end).unwrap()
    }
    
    point = Point::new(valid_start, usize::try_from(height).unwrap());

    for _el in valid_start..valid_end {
        draw_char(character, &point, array1);
        point.set_x(point.get_x() + 1);
    }
}

// this function places a said char in every space between a given range, only works in vertical lines, one at a time
pub fn draw_vertical_line (start: i32, end: i32, xpos: i32, array1: &mut [[char; CANVA.yu]; CANVA.xu], character: char) {

    let valid_start: usize;
    let valid_end: usize;
    let mut point: Point;

    if start > CANVA.y || end < 0 {
        return;} 
    if xpos < 0 || xpos >= CANVA.x {
        return;}

    if start < 0 {
        valid_start = 0;} 
    else {
        valid_start = usize::try_from(start).unwrap();}

    if end > CANVA.y {
        valid_end = CANVA.yu;} 
    else {
        valid_end = usize::try_from(end).unwrap()}

    point = Point::new(usize::try_from(xpos).unwrap(), valid_start);

    for _el in valid_start..valid_end {
        draw_char(character, &point, array1);
        point.set_x(point.get_x() + 1);
    }
}

// This function sets all characters in the canvas to an UTF8 blank char, otherwise all the spaces would be filled with 'a's. The blank char used: '⠀'
pub fn set_to_blank (array1: &mut [[char; CANVA.yu]; CANVA.xu]) {

    for i in 0..CANVA.y {
        draw_horizontal_line(0, CANVA.x, i, array1, ' ');
    }

}

// The start/end variables must be in a valid space, do not count the structural lines, you don't want to overwrite them
pub fn text_draw (start: i32, end: i32, height: i32, position: &str, array1: &mut [[char; CANVA.yu]; CANVA.xu], text: &str) {

    let mut string_start: i32;
    let string_chars: Vec<char> = str_to_char(text);
    let mut point: Point = Point::new(0, 0);

    if height >= CANVA.y {panic!("A height bigger than allowed was passed into text() !")}
    point.set_y(usize::try_from(height).unwrap());

    // The word is printed from left to right, with a counter to keep track of the last position. The starting position is defined in the block below
    match position {
        "left" => string_start = start,
 
        "right" => string_start = end - i32::try_from(text.len()).unwrap(),

        "center" => string_start = (start + end) / 2 - (i32::try_from(text.len()).unwrap() / 2),

        &_ => panic!("you put a invalid position in the text function!")
    }

    for character in string_chars {

        if string_start >= 0 && string_start < CANVA.x {
            point.set_x(usize::try_from(string_start).unwrap());
            draw_char(character, &point, array1);
        }
        string_start += 1;
    }
    
}

// The start/end variables must be in a valid space, do not count the structural lines, you don't want to overwrite them
// This does the same thing as fn text_draw(), but uses a array of chars instead of an &str
pub fn text_chars (start: i32, end: i32, height: i32, position: &str, array1: &mut [[char; CANVA.yu]; CANVA.xu], text: &[char]) {

    let mut string_start: i32;
    let string_chars: &[char] = text;
    let mut point: Point = Point::new(0, 0);

    if height >= CANVA.y {panic!("A height bigger than allowed was passed into text() !")}
    point.set_y(usize::try_from(height).unwrap());

    // The word is printed from left to right, with a counter to keep track of the last position. The starting position is defined in the block below
    match position {
        "left" => string_start = start,
 
        "right" => string_start = end - i32::try_from(text.len()).unwrap(),

        "center" => string_start = (start + end) / 2 - (i32::try_from(text.len()).unwrap() / 2),

        &_ => panic!("you put a invalid position in the text function!")
    }

    for num in 0..string_chars.len() {

        if string_start >= 0 {
            point.set_x(usize::try_from(string_start).unwrap());
            draw_char(string_chars[num], &point, array1);
        }
        string_start += 1;
    }

}

// Change: draw_ascii now takes a i32 as start value, this allow the function to be used with animations starting from outside the printing area
// Observation: sometimes you may see 199 being passed as end, this in just an arbitrary value, because most uses this isn't used, but 199 is also the biggest end value possible, so is the best arbitrary value to be used
pub fn draw_ascii (ascii: &[&str], start: i32, end: i32, mut height: i32, position: &str, array1: &mut [[char; 40]; 200]) {

    for el in ascii {

        if height >= 0 || height <= CANVA.y - 1 {
            text_draw (start, end, height, position, array1, el);
        }

        height += 1;

    }
}

pub fn text_with_line_breaker (max_len: usize, mut line_height: i32, start: i32, array1: &mut [[char; 40]; 200], text: &str) {

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

