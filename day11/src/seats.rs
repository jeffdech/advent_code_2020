use std::fmt;
use std::iter::Extend;

use im::Vector;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Occupied,
    Floor
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Floor => '.',
            Tile::Empty => 'L',
            Tile::Occupied => '#'
        };

        write!(f, "{}", c)
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Floor),
            '#' => Ok(Self::Occupied),
            'L' => Ok(Self::Empty),
            _ => Err("Unexpected character")
        }
    }
}

impl Tile {
    pub fn next<I>(self, neighbors: I) -> Self 
    where
        I: Iterator<Item = Self>,
    {
        match self {
            Self::Floor => Self::Floor,
            Self::Empty => match neighbors
                .filter(|t| matches!(t, Self::Occupied))
                .count()
            {
                0 => Self::Occupied,
                _ => Self::Empty
            },
            Self::Occupied => match neighbors
                .filter(|t| matches!(t, Self::Occupied))
                .count()
            {
                // Part I rule
                // (0..=3) => Self::Occupied,
                (0..=4) => Self::Occupied,
                _ => Self::Empty
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Vec2 {
    fn from(val: (isize, isize)) -> Self {
        Self {
            x: val.0,
            y: val.1
        }
    }
}

#[derive(Debug)]
pub struct Positioned<T>(Vec2, T);

#[derive(PartialEq, Clone)]
pub struct FloorPlan<T> 
where
    T: Clone
{
    pub size: Vec2,
    tiles: Vector<T>
}

impl<T> FloorPlan<T>
where
    T: Default + Clone
{
    pub fn new(size: Vec2) -> Self {
        let n_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..n_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect()
        }
    }
}

impl<T> FloorPlan<T>
where T: Clone
{
    pub fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + self.size.x * pos.y) as _)
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: Vec2, tile: T) {
        if let Some(idx) = self.index(pos) {
            self.tiles[idx] = tile;
        }
    }
}

impl<T> FloorPlan<T>
where
    T: Copy + PartialEq
{
    pub fn get(&self, pos: Vec2) -> Option<T> {
        self.index(pos).map(|idx| self.tiles[idx])
    }

    pub fn neighbor_tiles(&self, pos: Vec2) -> impl Iterator<Item = T> + '_ {
        self.neighbor_positions(pos).filter_map(move |pos| self.get(pos))
    }

    pub fn iter(&self) -> impl Iterator<Item = Positioned<T>> + '_ {
        (0..self.size.y)
            .map(move |y| {
                (0..self.size.x).map(move |x| {
                    let pos = Vec2::from((x, y));
                    Positioned(pos, self.get(pos).unwrap())
                })
            })
            .flatten()
    }

    fn count_type(&self, t: &T) -> usize {
        self.tiles
            .iter()
            .filter(|&x| x == t)
            .count()
    }
}

impl<T> FloorPlan<T> 
where T: Clone
{
    fn neighbor_positions(&self, pos: Vec2) -> impl Iterator<Item = Vec2> {
        (-1..=1)
            .map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .flatten()
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .map(move |(dx, dy)| {
                Vec2 {
                    x: pos.x + dx,
                    y: pos.y + dy
                }
            })
    }
}

impl<T> FloorPlan<T>
where
    T: TryFrom<char> + Clone
{
    pub fn parse(input: &str) -> Self where <T as TryFrom<char>>::Error: fmt::Debug {
        let mut rows = 1;
        let mut columns = 0;

        for c in input.chars() {
            if c == '\n' {
                rows += 1;
                columns = 0;
            } else {
                columns += 1;
            }
        }

        let chars = input.lines()
            .map(|l| l.chars())
            .flatten()
            .map(|c| T::try_from(c).unwrap())
            .collect();
        
        Self {
            size: (columns, rows).into(),
            tiles: chars
        }
    }
}

impl<T> fmt::Debug for FloorPlan<T>
where
    T: fmt::Debug + Copy + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{:?}", self.get((x, y).into()).unwrap())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<A> Extend<Positioned<A>> for FloorPlan<A> 
where A: Clone
{
    fn extend<T: IntoIterator<Item = Positioned<A>>>(&mut self, iter: T) {
        for Positioned(pos, tile) in iter {
            self.set(pos, tile)
        }
    }
}

impl FloorPlan<Tile> {
    pub fn next(&self) -> Self {
        let mut res = Self::new(self.size);
        res.extend(
            self.iter()
                .map(|Positioned(pos, tile)| Positioned(pos, tile.next(self.visible_seats(pos))))
                // Part I rule used neighbors not first visible
                // .map(|Positioned(pos, tile)| Positioned(pos, tile.next(self.neighbor_tiles(pos))))
        );
        res
    }

    pub fn last(self) -> Self {
        use itertools::Itertools;

        itertools::iterate(self, FloorPlan::next)
            .tuple_windows()
            .find_map(|(prev, next)| if prev == next { Some(next) } else { None })
            .unwrap()
    }

    pub fn empty_seats(self) -> usize {
        self.count_type(&Tile::Empty)
    }

    pub fn occupied_seats(self) -> usize {
        self.count_type(&Tile::Occupied)
    }

    pub fn visible_seats(&self, pos: Vec2) -> impl Iterator<Item = Tile> + '_ {
        use itertools::Itertools;

        (-1..=1)
            .map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .flatten()
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .map(move |(dx, dy)| {
                itertools::iterate(pos, move |v| Vec2 { x: v.x + dx, y: v.y + dy })
                    .skip(1)    // original position is the first of the iterator
                    .map(move |pos| self.index(pos))
                    .while_some()
                    .filter_map(move |idx| match self.tiles[idx] {
                        Tile::Floor => None,
                        seat => Some(seat)
                    })
                    .take(1)
            })
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_seats() {
        let plan = FloorPlan::<Tile>::parse(
            indoc::indoc!(
            "
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
            ")
            .trim()
        );

        assert_eq!(plan.visible_seats((3, 4).into()).count(), 8);
    }

    #[test]
    fn test_visible_seats2() {
        let plan = FloorPlan::<Tile>::parse(
            indoc::indoc!(
                "
                .##.##.
                #.#.#.#
                ##...##
                ...L...
                ##...##
                #.#.#.#
                .##.##.
                "
            )
            .trim()
        );

        // dbg!(plan.visible_seats((3, 3).into()).collect::<Vec<_>>());
        assert_eq!(plan.visible_seats((3, 3).into()).count(), 0);
    }
}