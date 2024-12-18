use num::{Unsigned, FromPrimitive, ToPrimitive};

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
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

    pub fn from_positions<T : Unsigned + Into<i32>>(from: (T, T), to: (T, T)) -> Option<Direction> {
        Direction::from_derivative((to.0.into() - from.0.into(), to.1.into() - from.1.into()))
    }

    pub fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::Top,
            '>' => Direction::Right,
            'v' => Direction::Bottom,
            '<' => Direction::Left,
            _ => panic!("Invalid direction char given")
        }
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

    pub fn step<T: Unsigned + PartialEq + ToPrimitive + FromPrimitive>(&self, position: (T, T)) -> Option<(T, T)> {
        if (position.0 == T::zero() && *self == Direction::Left) || (position.1 == T::zero() && *self == Direction::Top) { return None; }
        let dir = self.to_derivative();
        Some((T::from_i32(position.0.to_i32()? + dir.0)?, T::from_i32(position.1.to_i32()? + dir.1)?))
    }

    pub fn sort<T: Unsigned + ToPrimitive>(&self, positions: &mut Vec<(T, T)>) {
        let factors = self.to_derivative();
        positions.sort_by_key(|a| a.0.to_i32().expect("Int conversion failed") * factors.0 + a.1.to_i32().expect("Int conversion failed") * factors.1);
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

impl IntoIterator for Direction {
    type Item = Direction;

    type IntoIter = std::vec::IntoIter<Direction>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.clone(), self.turn_right(), self.turn_right().turn_right(), self.turn_left()].into_iter()
    }
}