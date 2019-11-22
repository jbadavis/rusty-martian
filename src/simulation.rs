use crate::rover::Position;
use crate::rover::Rover;

pub struct Simulation {
    pub grid: (i32, i32),
    pub rovers: Vec<Rover>,
    pub instructions: Vec<Vec<char>>,
    pub scents: Vec<Position>,
}

impl Simulation {
    pub fn new(grid: (i32, i32), rovers: Vec<Rover>, instructions: Vec<Vec<char>>) -> Simulation {
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
