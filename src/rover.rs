#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Position (pub i32, pub i32, pub char);

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
            'N' => if rotate_left { 'W' } else { 'E' },
            'E' => if rotate_left { 'N' } else { 'S' },
            'S' => if rotate_left { 'E' } else { 'W' },
            'W' => if rotate_left { 'S' } else { 'N' },
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
