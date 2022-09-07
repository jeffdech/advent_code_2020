use derive_more::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, Sub)]
pub struct Vec2 {
    x: isize,
    y: isize
}

impl Vec2 {
    pub fn manhattan_distance(self) -> usize {
        (self.x.abs() + self.y.abs()) as _
    }

    pub fn rotate(self, d: AngleDelta) -> Self {
        let Self { x, y } = self;
        let tp = match d.0.rem_euclid(4) {
            0 => (x,y),
            1 => (y, -x),
            2 => (-x, -y),
            3 => (-y, x),
            _ => unreachable!()
        };
        tp.into()
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(val: (isize, isize)) -> Self {
        Self {
            x: val.0,
            y: val.1
        }
    }
}

impl std::ops::Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Direction{
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Direction {
    pub fn vec(self) -> Vec2 {
        let tp = match self {
            Direction::East => (1, 0),
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::West => (-1, 0)
        };
        tp.into()
    }
}

impl Into<isize> for Direction {
    fn into(self) -> isize {
        self as _
    }
}

impl std::convert::TryFrom<isize> for Direction {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if (0..=3).contains(&value) {
            Ok(unsafe { std::mem::transmute(value as u8) })
        } else {
            Err("direction out of bounds!")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AngleDelta(isize);

impl std::ops::Add<AngleDelta> for Direction {
    type Output = Self;

    fn add(self, rhs: AngleDelta) -> Self::Output {
        use std::convert::TryInto;

        let angle: isize = self.into();
        (angle + rhs.0).rem_euclid(4).try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShipState {
    pub pos: Vec2,
    pub dir: Direction,
    pub waypoint: Vec2,
}

// Part I version
// impl std::ops::Add<Instruction> for ShipState {
//     type Output = Self;

//     fn add(self, rhs: Instruction) -> Self::Output {
//         match rhs {
//             Instruction::Move(dir, units) => Self {
//                 pos: self.pos + dir.vec() * units,
//                 ..self
//             },
//             Instruction::Rotate(delta) => Self {
//                 dir: self.dir + delta,
//                 ..self
//             },
//             Instruction::Advance(units) => Self {
//                 pos: self.pos + self.dir.vec() * units,
//                 ..self
//             }
//         }
//     }
// }

impl std::ops::Add<Instruction> for ShipState {
    type Output = Self;

    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Move(dir, units) => Self {
                waypoint: self.waypoint + dir.vec() * units,
                ..self
            },
            Instruction::Rotate(delta) => Self {
                waypoint: self.waypoint.rotate(delta),
                ..self
            },
            Instruction::Advance(units) => Self {
                pos: self.pos + self.waypoint * units,
                ..self
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    Move(Direction, isize),
    Rotate(AngleDelta),
    Advance(isize),
}

pub fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        use Instruction::*;
        use Direction::*;

        let cmd = line.chars().next().unwrap();
        let n: isize = (&line[1..]).parse().unwrap();

        match cmd {
            'N' => Move(North, n),
            'E' => Move(East, n),
            'S' => Move(South, n),
            'W' => Move(West, n),
            'F' => Advance(n),
            'L' => Rotate(AngleDelta(-n / 90)),
            'R' => Rotate(AngleDelta(n / 90)),
            c => panic!("Encountered unexpected command {}", c)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_add() {
        let a = Vec2{ x: 3, y: 4};
        let b = Vec2{ x: -1, y: 10};
        assert_eq!(a + b, Vec2{ x: 2, y: 14});
    }

    #[test]
    fn manhattan_example() {
        let start = Vec2 { x: 0, y: 0 };
        let end = Vec2 { x: 17, y: -8 };
        assert_eq!((end - start).manhattan_distance(), 25);
    }

    #[test]
    fn direction_try_from() {
        use std::convert::TryFrom;

        let try_func = <Direction as TryFrom<isize>>::try_from;

        assert_eq!(
            try_func(0).unwrap(),
            Direction::East
        );

        assert_eq!(
            try_func(2).unwrap(),
            Direction::West
        );

        assert!(try_func(-1).is_err());
        assert!(try_func(4).is_err());
    }

    #[test]
    fn test_direction_add() {
        assert_eq!(Direction::East + AngleDelta(1), Direction::South);
        assert_eq!(Direction::East + AngleDelta(-1), Direction::North);
        assert_eq!(Direction::East + AngleDelta(4), Direction::East);      
    }

    #[test]
    fn test_vec2_rotate() {
        let v = Vec2 { x: 3, y: 1 };
        assert_eq!(v.rotate(AngleDelta(0)), v);
        assert_eq!(v.rotate(AngleDelta(4)), v);
        assert_eq!(v.rotate(AngleDelta(-4)), v);
    
        assert_eq!(v.rotate(AngleDelta(1)), Vec2 { x: 1, y: -3 });
        assert_eq!(v.rotate(AngleDelta(2)), Vec2 { x: -3, y: -1 });
        assert_eq!(v.rotate(AngleDelta(3)), Vec2 { x: -1, y: 3 });
    }
}