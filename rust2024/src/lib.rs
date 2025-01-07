use std::fs::read_to_string;

pub fn read_grid(input_file: &str) -> Vec<Vec<char>> {
    let mut ans = vec![];
    let s = read_to_string(input_file).unwrap();
    for line in s.lines() {
        let row: Vec<char> = line.chars().collect();
        ans.push(row);
    }
    ans
}

pub fn print_grid(grid: &[Vec<char>]) {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Move {
    Up = b'^',
    Down = b'v',
    Left = b'<',
    Right = b'>',
}

impl From<Move> for char {
    fn from(val: Move) -> Self {
        val as u8 as char
    }
}
impl Move {
    pub fn from(c: char) -> Result<Move, char> {
        match c {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(c),
        }
    }

    pub fn do_move(&self, x: usize, y: usize) -> (i32, i32) {
        let (x, y) = (x as i32, y as i32);
        match self {
            Move::Up => (x - 1, y),
            Move::Down => (x + 1, y),
            Move::Left => (x, y - 1),
            Move::Right => (x, y + 1),
        }
    }
    pub fn turn(&self) -> Self {
        match self {
            Move::Up => Move::Right,
            Move::Down => Move::Left,
            Move::Left => Move::Up,
            Move::Right => Move::Down,
        }
    }
}

pub const DELTAS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
