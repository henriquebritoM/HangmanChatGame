use crate::CANVA;

pub struct Point {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: usize, y:usize) -> Point {
        Point{x, y}
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn set_x(&mut self, x:usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y:usize) {
        self.y = y;
    }
}

pub fn draw_char(character: char, point: &Point, array1: &mut [[char; 40]; 200]) {
    array1[point.x][point.y] = character;
}

// This is the main printing function, it cleans the terminal and then prints every element
pub fn print_terminal (array1: &[[char; CANVA.yu]; CANVA.xu]) {

    let mut height: usize = 0;
    print!("\n");

    println!("{esc}c", esc = 27 as char);  // This line doesen't  work very well

    while height < array1[0].len() {

        for num in 0..array1.len() {
            
            print!("{}", array1[num][height]);

        }
    
    print!("\n");

    height += 1;

    }
}

pub fn print_line (height: usize, array1: &[[char; CANVA.yu]; CANVA.xu]) {

    println!("\x1B[{};0H", height);

    for num in 0..array1.len() {
            
        print!("{}", array1[num][height - 1]);

    }
}