use crate::rover::{Position, Rover};

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
    Forward,
}

pub struct Grid(pub i32, pub i32);

pub struct Simulation {
    pub grid: Grid,
    pub rovers: Vec<Rover>,
    pub instructions: Vec<Vec<Instruction>>,
    pub scents: Vec<Position>,
}

impl Simulation {
    pub fn new(grid: Grid, rovers: Vec<Rover>, instructions: Vec<Vec<Instruction>>) -> Simulation {
        Simulation {
            grid,
            rovers,
            instructions,
            scents: vec![],
        }
    }

    pub fn instruction_moves_to_scent(&self, position: &Position) -> bool {
        self.scents.contains(&position)
    }

    pub fn add_scent(&mut self, position: Position) {
        self.scents.push(position);
    }

    pub fn off_grid(&self, position: &Position) -> bool {
        (position.0 > self.grid.0 || position.0 < 0) || (position.1 > self.grid.1 || position.1 < 0)
    }

    pub fn run(&mut self) {
        for i in 0..self.rovers.len() {
            for instruction in self.instructions[i].iter() {
                let new_position = self.rovers[i].follow_instruction(instruction);

                let moves_to_scent = self.instruction_moves_to_scent(&new_position);

                if self.off_grid(&new_position) && !moves_to_scent {
                    self.add_scent(new_position);
                    self.rovers[i].is_lost();

                    break;
                } else if moves_to_scent {
                    continue;
                } else {
                    self.rovers[i].accept_instruction();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rover::Orientation;

    #[test]
    fn check_simulation() {
        let grid = Grid(5, 3);

        let rover_one = Rover::new(Position(1, 1, Orientation::East));
        let rover_two = Rover::new(Position(3, 2, Orientation::North));
        let rover_three = Rover::new(Position(0, 3, Orientation::West));

        let instructions_one = vec![
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Forward,
        ];

        let instructions_two = vec![
            Instruction::Forward,
            Instruction::Right,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Forward,
            Instruction::Right,
            Instruction::Right,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Left,
        ];

        let instructions_three = vec![
            Instruction::Left,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Forward,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Left,
            Instruction::Forward,
            Instruction::Left,
        ];

        let rovers = vec![rover_one, rover_two, rover_three];
        let instructions = vec![instructions_one, instructions_two, instructions_three];

        let mut simulation = Simulation::new(grid, rovers, instructions);

        simulation.run();

        let result_positions: Vec<(Position, bool)> = simulation
            .rovers
            .iter()
            .map(|rover| (rover.position, rover.lost))
            .collect();

        assert_eq!(
            vec![
                (Position(1, 1, Orientation::East), false),
                (Position(3, 3, Orientation::North), true),
                (Position(2, 3, Orientation::South), false),
            ],
            result_positions
        );
    }
}
