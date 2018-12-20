use std::ops;

pub type AbsoluteLocation = (usize, usize);
pub type RelativeLocation = (isize, isize);

impl ops::Add<RelativeLocation> for AbsoluteLocation {
    type Output = AbsoluteLocation;

    fn add(self, _rhs: RelativeLocation) -> AbsoluteLocation {
        ((self.0 as isize + _rhs.0) as usize, (self.1 as isize + _rhs.1) as usize)
    }
}

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
