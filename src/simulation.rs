use crate::rover::Rover;
use crate::rover::Position;

pub struct Simulation {
    pub grid: (i32, i32),
    pub rovers: Vec<Rover>,
}

impl Simulation {
    pub fn new(grid: (i32, i32), rovers: Vec<Rover>) -> Simulation {
        Simulation { grid, rovers }
    }

    pub fn on_grid(&self, position: &Position) -> bool {
        (position.0 > self.grid.0 || position.0 < 0) || 
        (position.1 > self.grid.1 || position.1 < 0)
    }

    pub fn run(&mut self) {
        let new_position = self.rovers[0].follow_instruction();
        self.rovers[0].position = new_position;
        println!("Run: {:?}", self.rovers[0].position);

        let new_position = self.rovers[0].follow_instruction();
        self.rovers[0].position = new_position;
        println!("Run: {:?}", self.rovers[0].position);

        let new_position = self.rovers[0].follow_instruction();
        self.rovers[0].position = new_position;
        println!("Run: {:?}", self.rovers[0].position);
    }
}
