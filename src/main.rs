mod rover;
mod simulation;

use rover::Position;
use rover::Rover;
use simulation::Simulation;

fn main() {
    let grid = (5, 3);

    let rover_one = Rover::new(
        Position(1, 1, 'E'),
        vec!['R', 'F', 'R', 'F', 'R', 'F', 'R', 'F'],
    );
    let rover_two = Rover::new(
        rover::Position(3, 2, 'N'),
        vec![
            'F', 'R', 'R', 'F', 'L', 'L', 'F', 'F', 'R', 'R', 'F', 'L', 'L',
        ],
    );
    let rover_three = Rover::new(
        Position(0, 3, 'W'),
        vec!['L', 'L', 'F', 'F', 'F', 'L', 'F', 'L', 'F', 'L'],
    );

    let rovers = vec![rover_one, rover_two, rover_three];

    let mut simulation = Simulation::new(grid, rovers);

    simulation.run();

//     for rover in simulation.rovers.iter() {
//         println!("{:?}", rover.position);
//     }
}
