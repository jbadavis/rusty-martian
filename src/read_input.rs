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

fn get_grid(line: &str) -> Grid {
    let grid: Vec<i32> = line
        .chars()
        .filter(|c| c.clone() != ' ')
        .map(|c| char_to_int(c))
        .collect();

    Grid(grid[0], grid[1])
}

pub fn get_rovers(data: &Vec<&str>) -> Vec<Rover> {
    let mut rovers: Vec<Rover> = vec![];

    for line in (1..data.len()).step_by(2) {
        let rover_line: Vec<char> = data[line].chars().filter(|c| c.clone() != ' ').collect();

        let x = char_to_int(rover_line[0]);
        let y = char_to_int(rover_line[1]);

        rovers.push(Rover::new(Position(x, y, rover_line[2])));
    }

    rovers
}

pub fn get_instructions(data: &Vec<&str>) -> Vec<Vec<char>> {
    let mut instructions: Vec<Vec<char>> = vec![];

    for line in (2..data.len()).step_by(2) {
        let instruction_set: Vec<char> = data[line].chars().filter(|c| c.clone() != ' ').collect();

        instructions.push(instruction_set);
    }

    instructions
}
