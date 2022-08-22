use std::fmt;

#[derive(Debug, Copy, PartialEq, Clone)]
struct Vec2 {
    x: i64,
    y: i64
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Self {x, y}
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Tree
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
        write!(f, "{}", c)
    }
}

struct Map {
    size: Vec2,
    tiles: Vec<Tile>
}

impl Map {
    fn new(size: Vec2) -> Self {
        let n_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..n_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn set(&mut self, pos: Vec2, tile: Tile) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile;
        }
    }

    fn get(&self, pos: Vec2) -> Tile {
        self.index(pos).map(|i| self.tiles[i]).unwrap_or_default()
    }

    fn normalize_position(&self, pos: Vec2) -> Option<Vec2> {
        if pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            let x = if pos.x < 0 {
                self.size.x + (pos.x % self.size.x)
            } else {
                pos.x % self.size.x
            };
            Some((x, pos.y).into())
        }
    }

    fn index(&self, pos: Vec2) -> Option<usize> {
        self.normalize_position(pos)
            .map(|pos| (pos.x + pos.y * self.size.x) as _)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get(Vec2::from((col, row))));
            }
            writeln!(f);
        }

        Ok(())
    }
}

fn main() {
    let mut m = Map::new((6, 6).into());
    let pts = [(1, 1), (4, 1), (1, 3), (4, 3), (2, 4), (3, 4)];
    for p in (&pts).iter().copied() {
        m.set(p.into(), Tile::Tree);
    }

    println!("{:?}", m);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_map() -> Map {
        let mut m = Map::new((6, 6).into());
        let pts = [(1, 1), (4, 1), (1, 3), (4, 3), (2, 4), (3, 4)];
        for p in (&pts).iter().copied() {
            m.set(p.into(), Tile::Tree);
        }

        m
    }

    #[test]
    fn test_normalized_positions() {
        let mut m = default_map();

        assert_eq!(m.normalize_position((7, 2).into()), Some((1, 2).into()), "positive wrap");
        assert_eq!(m.normalize_position((-1, 2).into()), Some((5, 2).into()), "negative wrap");
    }
}