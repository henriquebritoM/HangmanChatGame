use std::{io::{self, stdout, Write}, option};

use crossterm::{self, cursor, style, QueueableCommand};

pub struct Point {
    x: i16,
    y: i16,
}

#[allow(dead_code)]
impl Point {
    pub const fn new(x: i16, y: i16) -> Point {
        Point { x, y }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }

    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }
    
}

#[allow(dead_code)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Debug)]
pub enum CursorError {
    CursorOutOfCanvas(String),
    PointTooBig(String)
}

#[derive(Debug)]
pub struct Cursor {
    blinking: bool,
    shown: bool,
}

impl Cursor {
    pub fn new(blinking: bool, shown: bool) -> Cursor {
        Cursor {blinking, shown}
    }  
    pub fn get_blink(&self) -> bool {
        self.blinking
    }
    pub fn set_blink(&mut self, blinking: bool) {
        self.blinking = blinking;
    }
    pub fn get_shown(&self) -> bool {
        self.shown
    }
    pub fn set_shown(&mut self, show: bool) {
        self.shown = show;
    }
    pub fn set_cursor(&self, canva: &Canva, point: &Point) -> Result<(), CursorError> {
        let mut stdout = io::stdout();

        if point.get_x() >= canva.get_width() 
            {return Err(CursorError::CursorOutOfCanvas(format!("tried to move cursor to {:?} on x axis", point.get_x())));}
        if point.get_y() >= canva.get_height() 
            {return Err(CursorError::CursorOutOfCanvas(format!("tried to move cursor to {:?} on y axis", point.get_y())));}

        let a: u16 = match u16::try_from(point.get_x()) {
            Ok(u16) => u16,
            Err(_) => {return Err(CursorError::PointTooBig(format!("tried to reach a negative point")));}
        };
        let b: u16 = match u16::try_from(point.get_y()) {
            Ok(u16) => u16,
            Err(_) => {return Err(CursorError::PointTooBig(format!("tried to reach a negative point")));}
        };

        stdout
            .queue(cursor::MoveTo(a, b))
            .unwrap();

        return Ok(());
    }

}

//use crate::CANVA;
#[derive(Debug)]
pub struct Canva {
    x: i16,
    y: i16,
    pub cursor: Cursor,

}

#[allow(dead_code)]
impl Canva {
    // creates the canva obj and the it's vector
    // it is a must to be able to convert i16 -> u16 due to crossterm 
    pub fn new(width: i16, height: i16, cursor: Cursor) -> Canva {

        match u16::try_from(width) {  
            Ok(u16) => u16,
            Err(e) => panic!("{}", e)
        };

        match u16::try_from(height) {
            Ok(u16) => u16,
            Err(e) => panic!("{}", e)
        };
        
        Canva {
            x: width,
            y: height,
            cursor: cursor

        }
    }

    pub fn get_width(&self) -> i16 {
        self.x
    }

    pub fn get_height(&self) -> i16 {
        self.y
    }

    pub fn flush(&self) {
        let mut stdout = stdout();
        stdout.flush().unwrap();
    }

    pub fn clear_terminal() {
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All);
    }

    pub fn draw_char(&self, c: char, p: &Point) {
        match self.cursor.set_cursor(self, &p) {
            Ok(_) => {let mut stdout = io::stdout();
                      stdout.queue(style::Print(c)).unwrap();},
            Err(_) => {}
        }
    }

    pub fn set_to_blank(&self) {
        for h in 0..self.get_height() {
            self.draw_horizontal_line(Point::new(0, h), self.get_width(), ' ');
        }
    }

    pub fn draw_horizontal_line(&self, left_point: Point, lenght: i16, character: char) {
        
        if left_point.get_y() > self.get_height() || left_point.get_y() < 0 {
            return;
        } // do nothing, horizontal line either below canvas or above it
        if left_point.get_x() > self.get_width() {
            return;
        } // do nothing, left point already after the right border of canvas
        if left_point.get_x() + lenght < 0 {
            return;
        } // do nothing, line ends before the leftmost part of canvas
        
        let mut offset: i16 = 0;
        let mut valid_lenght: i16;
        
        if left_point.get_x() < 0 {
            offset = 0 - left_point.get_x();
        }

        let mut valid_left_point: Point = Point::new(left_point.get_x() - offset, left_point.get_y());
        
        valid_lenght = lenght - offset;

        if valid_lenght + valid_left_point.get_x() > self.get_width() {
            valid_lenght = self.get_width() - valid_left_point.get_x() - 1;
        }

        for _num in 0..valid_lenght {
            self.draw_char(character, &valid_left_point);
            valid_left_point.set_x(valid_left_point.get_x() + 1);
        }

        self.flush();
    }

    pub fn draw_vertical_line(&self, top_point: Point, lenght: i16, character: char) {

        if top_point.get_x() > self.get_width() || top_point.get_x() < 0 {
            return;
        } // do nothing, vertical line before or after canvas
        if top_point.get_y() > self.get_height() {
            return;
        } // do nothing, top point already below the canvas
        if top_point.get_y() + lenght < 0 {
            return;
        } // do nothing, line ends before the topmost part of canvas

        let mut offset: i16 = 0;
        let mut valid_lenght: i16;
        
        if top_point.get_y() < 0 {
            offset = 0 - top_point.get_y();
        }

        let mut valid_top_point: Point = Point::new(top_point.get_x(), top_point.get_y() - offset);

        valid_lenght = lenght - offset;

        if valid_lenght + valid_top_point.get_y() > self.get_height() {
            valid_lenght = self.get_height() - valid_top_point.get_y() - 1;
        }

        for _num in 0..valid_lenght {
            self.draw_char(character, &valid_top_point);
            valid_top_point.set_y(valid_top_point.get_y() + 1);
        }

        self.flush();
    }

    // alignment define where the point is in the world
    pub fn text_draw(&self, ref_point: &Point, align: &Alignment, text: &str) {
        let string_chars: Vec<char> = str_to_char(text);
        let mut copied_point = Point::new(ref_point.get_x(), ref_point.get_y());
        let offset: i16;

        match align {
            Alignment::Left => offset = 0,

            Alignment::Right => offset = i16::try_from(string_chars.len()).unwrap(),

            Alignment::Center => offset = i16::try_from(string_chars.len()).unwrap() / 2,
        }

        copied_point.set_x(ref_point.get_x() - offset);

        for character in string_chars {
            self.draw_char(character, &copied_point);
            copied_point.set_x(copied_point.get_x() + 1);
        }

        self.flush();
    }

    // fn dedicated to centralize text, takes a point and a distance
    pub fn text_cdraw(&self, ref_point: &Point, max_len: i16, text: &str) {
        let string_chars: Vec<char> = str_to_char(text);
        let mut copied_point = Point::new(ref_point.get_x(), ref_point.get_y());
        let offset: i16;

        offset = max_len - ref_point.get_x() / 2 - i16::try_from(string_chars.len()).unwrap() / 2;

        copied_point.set_x(ref_point.get_x() + offset);

        for character in string_chars {
            self.draw_char(character, &copied_point);
            copied_point.set_x(copied_point.get_x() + 1);
        }

        self.flush();
    }

    pub fn draw_ascii(&self, ascii: &[&str], top_left_point: &Point, align: &Alignment) {

        let mut c_point = Point::new(top_left_point.get_x(), top_left_point.get_y());

        for el in ascii {
            if c_point.get_y() < self.get_height() {
                self.text_draw(&c_point, align, *el);
            }

            c_point.set_y(c_point.get_y() + 1);
        }
    }

    pub fn text_with_line_breaker(&self, start: &Point, max_len: usize,  text: &str) {
        let word_chars: Vec<char> = str_to_char(text);
        let mut c_point: Point = Point::new(start.get_x(), start.get_y());
        let mut pos: usize = 0;
        let mut temp_str: String = String::new();

        loop {
            if (word_chars.len() - pos) <= max_len {
                for i in pos..word_chars.len() {
                    temp_str.push(word_chars[i])
                }

                self.text_draw(&c_point, &Alignment::Left, &temp_str);
                break;
            } else {
                for k in (pos..max_len + pos).rev() {
                    if word_chars[k] == ' ' {
                        for e in pos..k {
                            temp_str.push(word_chars[e])
                        }

                        self.text_draw(&c_point, &Alignment::Left, &temp_str);
                        pos = k + 1; // This skips the ' ' char
                        c_point.set_y(c_point.get_y() + 1);
                        temp_str.clear();
                        break;
                    }
                }
            }
        }
    }
}

pub fn str_to_char(string: &str) -> Vec<char> {
    let mut char_vec = Vec::new();

    for character in string.chars() {
        char_vec.push(character)
    }

    char_vec
}

