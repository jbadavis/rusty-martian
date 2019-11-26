use std::fs::File;
use std::io::Read;

use crate::rover::{Orientation, Position, Rover};
use crate::simulation::{Grid, Instruction};

const RADIX: u32 = 10;

pub fn read_input() -> (Grid, Vec<Rover>, Vec<Vec<Instruction>>) {
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

fn parse_orientation(orientation: char) -> Result<Orientation, String> {
    match orientation {
        'E' => Ok(Orientation::East),
        'N' => Ok(Orientation::North),
        'S' => Ok(Orientation::South),
        'W' => Ok(Orientation::West),
        _ => Err(String::from("unknown rover orientation")),
    }
}

fn parse_instruction(instruction: char) -> Result<Instruction, String> {
    match instruction {
        'L' => Ok(Instruction::Left),
        'R' => Ok(Instruction::Right),
        'F' => Ok(Instruction::Forward),
        _ => Err(String::from("unknown instruction")),
    }
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

        let orientation = parse_orientation(parsed_line[2]).expect("get_rovers");

        rovers.push(Rover::new(Position(x, y, orientation)));
    }

    rovers
}

fn get_instructions(data: &Vec<&str>) -> Vec<Vec<Instruction>> {
    let mut instructions: Vec<Vec<Instruction>> = vec![];

    for line in (2..data.len()).step_by(2) {
        let parsed_line = parse_line(data[line]);

        let mut parsed_instructions: Vec<Instruction> = vec![];

        for c in parsed_line {
            let instruction = parse_instruction(c).expect("get_rovers");

            parsed_instructions.push(instruction);
        }

        instructions.push(parsed_instructions);
    }

    instructions
}
