mod rover;
mod simulation;

use rover::Position;
use rover::Rover;
use simulation::Simulation;

fn main() {
    let grid = (5, 3);

    let rover_one = Rover::new(Position(1, 1, 'E'));
    let rover_two = Rover::new(Position(3, 2, 'N'));
    let rover_three = Rover::new(Position(0, 3, 'W'));

    let instructions_one = vec!['R', 'F', 'R', 'F', 'R', 'F', 'R', 'F'];
    let instructions_two = vec!['F', 'R', 'R', 'F', 'L', 'L', 'F', 'F', 'R', 'R', 'F', 'L', 'L'];
    let instructions_three = vec!['L', 'L', 'F', 'F', 'F', 'L', 'F', 'L', 'F', 'L'];

    let rovers = vec![rover_one, rover_two, rover_three];
    let instructions = vec![instructions_one, instructions_two, instructions_three];

    let mut simulation = Simulation::new(grid, rovers, instructions);

    simulation.run();
}
