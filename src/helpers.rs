#[derive(PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal
}

#[derive(PartialEq)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left
}
impl Direction {
    pub fn from_derivative(dir: (i32, i32)) -> Option<Direction> {
        match dir {
            (0, -1) => Some(Direction::Top),
            (1, 0) => Some(Direction::Right),
            (0, 1) => Some(Direction::Bottom),
            (-1, 0) => Some(Direction::Left),
            _ => None
        }
    }
    pub fn to_derivative(&self) -> (i32, i32) {
        match self {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0)
        }
    }

    pub fn from_positions(from: (u16, u16), to: (u16, u16)) -> Option<Direction> {
        Direction::from_derivative((to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32))
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Top => Direction::Left,
            Direction::Right => Direction::Top,
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Bottom
        }
    }
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top
        }
    }
    pub fn flip(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right
        }
    }

    pub fn step(&self, position: (u16, u16)) -> Option<(u16, u16)> {
        if (position.0 == 0 && *self == Direction::Left) || (position.1 == 0 && *self == Direction::Top) { return None; }
        let dir = self.to_derivative();
        Some(((position.0 as i32 + dir.0) as u16, (position.1 as i32 + dir.1) as u16))
    }

    pub fn sort(&self, positions: &mut Vec<(u16, u16)>) {
        let factors = self.to_derivative();
        positions.sort_by_key(|a| a.0 as i32 * factors.0 + a.1 as i32 * factors.1);
    }

    pub fn orientation(&self) -> Orientation {
        match self {
            Direction::Top => Orientation::Vertical,
            Direction::Right => Orientation::Horizontal,
            Direction::Bottom => Orientation::Vertical,
            Direction::Left => Orientation::Horizontal,
        }
    }
}