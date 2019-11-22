#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32, pub char);

#[derive(Debug)]
pub struct Rover {
    pub position: Position,
    pub next_position: Position,
    pub lost: bool,
}

pub enum Direction {
    Left,
    Right,
}

impl Rover {
    pub fn new(position: Position) -> Rover {
        Rover {
            lost: false,
            next_position: Position(0, 0, 'N'),
            position,
        }
    }

    pub fn rotate(&mut self, direction: Direction) -> Position {
        let Position(x, y, orientation) = self.position;

        let rotate_left = match direction {
            Direction::Left => true,
            Direction::Right => false,
        };

        let orientation = match orientation {
            'N' => {
                if rotate_left {
                    'W'
                } else {
                    'E'
                }
            }
            'E' => {
                if rotate_left {
                    'N'
                } else {
                    'S'
                }
            }
            'S' => {
                if rotate_left {
                    'E'
                } else {
                    'W'
                }
            }
            'W' => {
                if rotate_left {
                    'S'
                } else {
                    'N'
                }
            }
            _ => orientation,
        };

        Position(x, y, orientation)
    }

    pub fn move_forward(&mut self) -> Position {
        let Position(x, y, orientation) = self.position;

        let position = match self.position.2 {
            'N' => Position(x, y + 1, orientation),
            'E' => Position(x + 1, y, orientation),
            'S' => Position(x, y - 1, orientation),
            'W' => Position(x - 1, y, orientation),
            _ => self.position,
        };

        position
    }

    pub fn accept_instruction(&mut self) {
        self.position = self.next_position;
    }

    pub fn follow_instruction(&mut self, instruction: &char) -> Position {
        let position = match instruction {
            'L' => self.rotate(Direction::Left),
            'R' => self.rotate(Direction::Right),
            'F' => self.move_forward(),
            _ => self.position,
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
        let mut rover_north = Rover::new(Position(2, 2, 'N'));
        let mut rover_east = Rover::new(Position(2, 2, 'E'));
        let mut rover_south = Rover::new(Position(2, 2, 'S'));
        let mut rover_west = Rover::new(Position(2, 2, 'W'));

        assert_eq!(
            [
                Position(2, 2, 'W'),
                Position(2, 2, 'N'),
                Position(2, 2, 'E'),
                Position(2, 2, 'S')
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
        let mut rover_north = Rover::new(Position(2, 2, 'N'));
        let mut rover_east = Rover::new(Position(2, 2, 'E'));
        let mut rover_south = Rover::new(Position(2, 2, 'S'));
        let mut rover_west = Rover::new(Position(2, 2, 'W'));

        assert_eq!(
            [
                Position(2, 2, 'E'),
                Position(2, 2, 'S'),
                Position(2, 2, 'W'),
                Position(2, 2, 'N')
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
        let mut rover_north = Rover::new(Position(2, 2, 'N'));
        let mut rover_east = Rover::new(Position(2, 2, 'E'));
        let mut rover_south = Rover::new(Position(2, 2, 'S'));
        let mut rover_west = Rover::new(Position(2, 2, 'W'));

        assert_eq!(
            [
                Position(2, 3, 'N'),
                Position(3, 2, 'E'),
                Position(2, 1, 'S'),
                Position(1, 2, 'W')
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
    fn position_accepted() {
        let mut rover = Rover::new(Position(2, 2, 'N'));

        rover.move_forward();
        rover.accept_instruction();

        assert_eq!(rover.next_position, rover.position);
    }

    #[test]
    fn next_position_set() {
        let mut rover = Rover::new(Position(1, 1, 'E'));

        rover.follow_instruction(&'F');

        assert_eq!(Position(2, 1, 'E'), rover.next_position);
    }
}
