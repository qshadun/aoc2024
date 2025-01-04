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
