mod tiles;
use crate::tiles::*;

fn main() {
    let result = parser::parse_tileset(include_str!("example.txt")).unwrap().1;

    for t in result {
        println!("{}", t);
    }
}
