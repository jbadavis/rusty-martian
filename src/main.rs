mod rover;
mod simulation;
mod read_input;

use crate::read_input::read_input;
use crate::simulation::Simulation;
use crate::rover::Position;

fn main() {
    let (grid, rovers, instructions) = read_input();
    
    let mut simulation = Simulation::new(grid, rovers, instructions);

    simulation.run();

    for rover in simulation.rovers.iter() {
        let Position(x, y, orientation) = rover.position;

        if rover.lost {
            println!("{} {} {:?} LOST", x, y, orientation);
        } else {
            println!("{} {} {:?}", x, y, orientation);
        }
    }
}
