pub type AbsoluteLocation = (usize, usize);
pub type RelativeLocation = (isize, isize);

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn location(&self) -> RelativeLocation {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }

    pub fn from_num(input: i32) -> Option<Direction> {
        match input {
            0 => Some(Direction::Left),
            1 => Some(Direction::Right),
            2 => Some(Direction::Up),
            3 => Some(Direction::Down),
            4 => Some(Direction::UpLeft),
            5 => Some(Direction::UpRight),
            6 => Some(Direction::DownLeft),
            7 => Some(Direction::DownRight),
            _ => None,
        }
    }
}
