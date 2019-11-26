use std::fs::File;
use std::io::Read;

use crate::rover::{Position, Rover};
use crate::simulation::Grid;

const RADIX: u32 = 10;

pub fn read_input() -> (Grid, Vec<Rover>, Vec<Vec<char>>) {
    let data = match open_file() {
        Err(error) => panic!("Couldn't read file: {:?}", error),
        Ok(contents) => contents,
    };

    let data: Vec<&str> = data.split('\n').filter(|line| !line.is_empty()).collect();

    (
        get_grid(data[0]),
        get_rovers(&data),
        get_instructions(&data),
    )
}

fn open_file() -> std::io::Result<String> {
    let mut f = File::open("data.txt")?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn char_to_int(c: char) -> i32 {
    c.to_digit(RADIX).unwrap() as i32
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_grid(line: &str) -> Grid {
    let grid: Vec<i32> = line
        .chars()
        .filter(|c| c.clone() != ' ')
        .map(|c| char_to_int(c))
        .collect();

    Grid(grid[0], grid[1])
}

fn get_rovers(data: &Vec<&str>) -> Vec<Rover> {
    let mut rovers: Vec<Rover> = vec![];

    for line in (1..data.len()).step_by(2) {
        let parsed_line = parse_line(data[line]);

        let x = char_to_int(parsed_line[0]);
        let y = char_to_int(parsed_line[1]);

        rovers.push(Rover::new(Position(x, y, parsed_line[2])));
    }

    rovers
}

fn get_instructions(data: &Vec<&str>) -> Vec<Vec<char>> {
    let mut instructions: Vec<Vec<char>> = vec![];

    for line in (2..data.len()).step_by(2) {
        instructions.push(parse_line(data[line]));
    }

    instructions
}
