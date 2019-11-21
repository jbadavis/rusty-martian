use crate::rover::Position;
use crate::rover::Rover;

pub struct Simulation {
    pub grid: (i32, i32),
    pub rovers: Vec<Rover>,
    pub instructions: Vec<Vec<char>>,
}

impl Simulation {
    pub fn new(grid: (i32, i32), rovers: Vec<Rover>, instructions: Vec<Vec<char>>) -> Simulation {
        Simulation {
            grid,
            rovers,
            instructions,
        }
    }

    pub fn on_grid(&self, position: &Position) -> bool {
        (position.0 > self.grid.0 || position.0 < 0) || (position.1 > self.grid.1 || position.1 < 0)
    }

    pub fn run(&mut self) {
        for i in 0..self.rovers.len() {
            for instruction in self.instructions[0].iter() {
                let new_position = self.rovers[i].follow_instruction(instruction);

                if self.on_grid(&new_position) {
                    self.rovers[i].accept_move();
                }
            }
        }
    }
}
