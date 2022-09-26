mod cubes;
use crate::cubes::*;

fn main() {
    // let mut grid = CubeGrid::<Vec4>::parse(include_str!("example.txt"));
    let mut grid = CubeGrid::<Vec4>::parse(include_str!("input.txt"));

    // println!("{}", grid);
    (1..=6)
        .for_each(|i| {
            grid = grid.next();
            // println!("======== Turn {} =======", i);
            // println!("{}", grid);
        });

    println!("After six turns, there are {} cubes", grid.active_cubes());
}
