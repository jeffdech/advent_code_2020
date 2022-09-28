use std::fmt;

use array2d::Array2D;

use nom::{
    bytes::complete::{tag},
    character::complete::{digit1, newline, one_of},
    combinator::{map, opt},
    error::Error,
    multi::{many1},
    sequence::{tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellState {
    On,
    Off,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            CellState::On => '#',
            CellState::Off => '.'
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
pub enum Flip {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq)]
pub struct Tile {
    num: usize,
    contents: Array2D<CellState>
}

impl Tile {
    pub fn new(num: usize, contents: Array2D<CellState>) -> Self {
        Self { num, contents }
    }

    pub fn edge(&self, dir: Direction) -> Vec<CellState> {
        match dir {
            Direction::North => self.contents.row_iter(0).map(|&v| v).collect(),
            Direction::South => self.contents.row_iter(9).map(|&v| v).collect(),
            Direction::East => self.contents.column_iter(9).map(|&v| v).collect(),
            Direction::West => self.contents.column_iter(0).map(|&v| v).collect()
        }
    }

    pub fn matches_edge(&self, other: &Self, dir: Direction) -> bool {
        let dother = match dir {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        };

        self.edge(dir) == other.edge(dother)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile: {}\n", self.num);
        for row in self.contents.columns_iter() {
            for val in row {
                write!(f, "{}", val);
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

pub mod parser {
    use super::*;

    type Res<T, U> = IResult<T, U, Error<T>>;

    pub fn parse_tileset(input: &str) -> Res<&str, Vec<Tile>> {
        many1(parse_tile)(input)
    }

    pub fn parse_tile(input: &str) -> Res<&str, Tile> {
        let header = map(
            tuple((tag::<&str, &str, Error<&str>>("Tile "), digit1, tag(":"), opt(newline))),
            |(_, d, _, _)| d.parse().unwrap()
        );

        map(
            tuple((header, many1(parse_line), opt(newline))),
            |(n, cts, _)| Tile::new(n, Array2D::from_rows(&cts))
        )(input)
    }

    pub fn parse_line(input: &str) -> Res<&str, Vec<CellState>> {
        map(
            tuple((many1(one_of(".#")), opt(newline))),
            |(cs, _)| {
                cs.into_iter()
                .map(|c| match c {
                    '.' => CellState::Off,
                    '#' => CellState::On,
                    _ => unreachable!()
                })
                .collect()
            }
        )(input)
    }
}