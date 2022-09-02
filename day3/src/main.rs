use std::fmt;
use std::ops::AddAssign;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Tree,
    Open
}

impl Default for Tile {
    fn default() -> Self {
        Self::Open
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Open => '.',
            Tile::Tree => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Map {
    size: Vec2,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(size: Vec2) -> Self {
        let n = size.x * size.y;

        Self {
            size,
            tiles: (0..n)
                    .into_iter()
                    .map(|_| Default::default())
                    .collect(),
        }
    }

    fn set(&mut self, pos: Vec2, tile: Tile) {
        if let Some(i) = self.pos_to_index(pos) {
            self.tiles[i] = tile
        }
    }

    fn get(&self, pos: Vec2) -> Tile {
        self.pos_to_index(pos)
            .map(|i| self.tiles[i])
            .unwrap_or_default()
    }

    fn parse(input: &[u8]) -> Self {
        let mut columns = 0;
        let mut rows = 0;

        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                columns = 0;
            } else {
                columns += 1;
            }
        }
        rows += 1;

        let mut iter = input.iter().copied();
        let mut map = Map::new((columns, rows).into());

        for r in 0..rows {
            for c in 0..columns {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Open,
                    Some(b'#') => Tile::Tree,
                    c => panic!("Expected '.' or '#', got {:?}", c),
                };

                map.set((c, r).into(), tile);
            }
            iter.next();
        }

        map
    }

    fn normalize_pos(&self, pos: Vec2) -> Option<Vec2> {
        if pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            let xmod = pos.x % self.size.x;
            let xout = if xmod < 0 { xmod + self.size.x } else { xmod };
            Some((xout, pos.y).into())
        }
    }

    fn pos_to_index(&self, pos: Vec2) -> Option<usize> {
        self.normalize_pos(pos)
            .map(|p| (p.x + p.y * self.size.x) as _)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get((col, row).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn make_path(map: &Map, delta: Vec2) -> Vec<Vec2> {
    let mut pos: Vec2 = (0, 0).into();
    let mut res: Vec<_> = Default::default();

    while map.normalize_pos(pos).is_some() {
        res.push(pos);
        pos += delta;
    }
    
    res
}

fn main() {
    let map = Map::parse(include_bytes!("input.txt"));

    println!("======= PART ONE =======");
    println!("Size of map is ({} x {})", map.size.x, map.size.y);
    let path = make_path(&map, (3, 1).into());
    
    let trees = path.into_iter().filter(|&v| map.get(v) == Tile::Tree).count();
    println!("Number of trees is {trees}");

    println!("======= PART TWO ========");
    let deltas = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let deltas: Vec<Vec<Vec2>> = deltas.into_iter().map(|d| make_path(&map, d.into())).collect();
    let tree_counts: usize = deltas.into_iter()
        .map(|path| path.into_iter().filter(|&p| map.get(p) == Tile::Tree).count())
        .product();

    println!("The total product is {tree_counts}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_map() -> Map {
        Map {
            size: (10, 10).into(),
            tiles: Vec::new()
        }
    }

    #[test]
    fn test_tuple() {
        let v: Vec2 = (5, 8).into();
        assert_eq!(v.x, 5);
        assert_eq!(v.y, 8);
    }

    #[test]
    fn test_normalize_pos() {
        let map = default_map();

        assert_eq!(map.normalize_pos((9, -9).into()), None);
        assert_eq!(map.normalize_pos((5, 11).into()), None);
        assert_eq!(map.normalize_pos((3, 4).into()), Some((3, 4).into()));
        assert_eq!(map.normalize_pos((-2, 5).into()), Some((8, 5).into()));
        assert_eq!(map.normalize_pos((12, 5).into()), Some((2, 5).into()));
    }

    #[test]
    fn test_pos_to_index() {
        let map = default_map();

        assert_eq!(map.pos_to_index((3, 2).into()), Some(23));
        assert_eq!(map.pos_to_index((0, 5).into()), Some(50));
    }
}