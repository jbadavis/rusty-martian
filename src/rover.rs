use crate::simulation::Instruction;

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Orientation {
    East,
    North,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32, pub Orientation);

#[derive(Debug)]
pub struct Rover {
    pub position: Position,
    pub next_position: Position,
    pub lost: bool,
}

impl Rover {
    pub fn new(position: Position) -> Rover {
        Rover {
            lost: false,
            next_position: Position(0, 0, Orientation::North),
            position,
        }
    }

    pub fn rotate(&mut self, direction: Direction) -> Position {
        let Position(x, y, orientation) = self.position;

        let orientation = match (orientation, direction) {
            (Orientation::North, Direction::Left) => Orientation::West,
            (Orientation::North, Direction::Right) => Orientation::East,
            (Orientation::East, Direction::Left) => Orientation::North,
            (Orientation::East, Direction::Right) => Orientation::South,
            (Orientation::South, Direction::Left) => Orientation::East,
            (Orientation::South, Direction::Right) => Orientation::West,
            (Orientation::West, Direction::Left) => Orientation::South,
            (Orientation::West, Direction::Right) => Orientation::North,
        };

        Position(x, y, orientation)
    }

    pub fn move_forward(&mut self) -> Position {
        let Position(x, y, orientation) = self.position;

        match self.position.2 {
            Orientation::North => Position(x, y + 1, orientation),
            Orientation::East => Position(x + 1, y, orientation),
            Orientation::South => Position(x, y - 1, orientation),
            Orientation::West => Position(x - 1, y, orientation),
        }
    }

    pub fn is_lost(&mut self) {
        self.lost = true;
    }

    pub fn accept_instruction(&mut self) {
        self.position = self.next_position;
    }

    pub fn follow_instruction(&mut self, instruction: &Instruction) -> Position {
        let position = match instruction {
            Instruction::Left => self.rotate(Direction::Left),
            Instruction::Right => self.rotate(Direction::Right),
            Instruction::Forward => self.move_forward(),
        };

        self.next_position = position;

        position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_when_left() {
        let mut rover_north = Rover::new(Position(2, 2, Orientation::North));
        let mut rover_east = Rover::new(Position(2, 2, Orientation::East));
        let mut rover_south = Rover::new(Position(2, 2, Orientation::South));
        let mut rover_west = Rover::new(Position(2, 2, Orientation::West));

        assert_eq!(
            [
                Position(2, 2, Orientation::West),
                Position(2, 2, Orientation::North),
                Position(2, 2, Orientation::East),
                Position(2, 2, Orientation::South)
            ],
            [
                rover_north.rotate(Direction::Left),
                rover_east.rotate(Direction::Left),
                rover_south.rotate(Direction::Left),
                rover_west.rotate(Direction::Left)
            ]
        );
    }

    #[test]
    fn rotate_when_right() {
        let mut rover_north = Rover::new(Position(2, 2, Orientation::North));
        let mut rover_east = Rover::new(Position(2, 2, Orientation::East));
        let mut rover_south = Rover::new(Position(2, 2, Orientation::South));
        let mut rover_west = Rover::new(Position(2, 2, Orientation::West));

        assert_eq!(
            [
                Position(2, 2, Orientation::East),
                Position(2, 2, Orientation::South),
                Position(2, 2, Orientation::West),
                Position(2, 2, Orientation::North)
            ],
            [
                rover_north.rotate(Direction::Right),
                rover_east.rotate(Direction::Right),
                rover_south.rotate(Direction::Right),
                rover_west.rotate(Direction::Right)
            ]
        );
    }

    #[test]
    fn move_foward() {
        let mut rover_north = Rover::new(Position(2, 2, Orientation::North));
        let mut rover_east = Rover::new(Position(2, 2, Orientation::East));
        let mut rover_south = Rover::new(Position(2, 2, Orientation::South));
        let mut rover_west = Rover::new(Position(2, 2, Orientation::West));

        assert_eq!(
            [
                Position(2, 3, Orientation::North),
                Position(3, 2, Orientation::East),
                Position(2, 1, Orientation::South),
                Position(1, 2, Orientation::West)
            ],
            [
                rover_north.move_forward(),
                rover_east.move_forward(),
                rover_south.move_forward(),
                rover_west.move_forward()
            ]
        );
    }

    #[test]
    fn rover_is_lost() {
        let mut rover = Rover::new(Position(2, 2, Orientation::North));

        rover.is_lost();

        assert_eq!(rover.lost, true);
    }

    #[test]
    fn position_accepted() {
        let mut rover = Rover::new(Position(2, 2, Orientation::North));

        rover.move_forward();
        rover.accept_instruction();

        assert_eq!(rover.next_position, rover.position);
    }

    #[test]
    fn next_position_set() {
        let mut rover = Rover::new(Position(1, 1, Orientation::East));

        rover.follow_instruction(&Instruction::Forward);

        assert_eq!(Position(2, 1, Orientation::East), rover.next_position);
    }
}
