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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_simulation() {
        let grid = (5, 3);

        let rover_one = Rover::new(Position(1, 1, 'E'));
        let rover_two = Rover::new(Position(3, 2, 'N'));
        let rover_three = Rover::new(Position(0, 3, 'W'));

        let instructions_one = vec!['R', 'F', 'R', 'F', 'R', 'F', 'R', 'F'];
        let instructions_two = vec![
            'F', 'R', 'R', 'F', 'L', 'L', 'F', 'F', 'R', 'R', 'F', 'L', 'L',
        ];
        let instructions_three = vec!['L', 'L', 'F', 'F', 'F', 'L', 'F', 'L', 'F', 'L'];

        let rovers = vec![rover_one, rover_two, rover_three];
        let instructions = vec![instructions_one, instructions_two, instructions_three];

        let mut simulation = Simulation::new(grid, rovers, instructions);

        simulation.run();

        let result_positions: Vec<Position> = simulation
            .rovers
            .iter()
            .map(|rover| rover.position)
            .collect();

        assert_eq!(
            vec![
                Position(1, 1, 'E'),
                Position(3, 3, 'N'),
                Position(2, 3, 'S')
            ],
            result_positions
        );
    }
}
