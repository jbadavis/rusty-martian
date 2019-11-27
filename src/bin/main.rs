use rusty_martian::read_input::read_input;
use rusty_martian::rover::Position;
use rusty_martian::simulation::Simulation;

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
