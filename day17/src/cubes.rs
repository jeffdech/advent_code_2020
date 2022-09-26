use std::collections::HashMap;
use std::fmt;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug, Ord, Hash)]
pub struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

type Vec3Tuple = (isize, isize, isize);
impl From<Vec3Tuple> for Vec3 {
    fn from(t: Vec3Tuple) -> Self {
        Vec3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl Vec3 {
    pub fn neighbors(&self) -> Vec<Vec3> {
        let deltas = (-1..=1).cartesian_product((-1..=1).cartesian_product(-1..=1));
        deltas
            .into_iter()
            .filter(|&(dx, (dy, dz)): &(isize, (isize, isize))| (dx.abs() + dy.abs() + dz.abs()) > 0)
            .map(|(dx, (dy, dz))| {
                Vec3 {
                    x: self.x + dx,
                    y: self.y + dy,
                    z: self.z + dz
                }
            })
            .collect()
    }

    pub fn abs(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CubeState {
    Active,
    Inactive
}

impl fmt::Display for CubeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match *self {
            CubeState::Active => '#',
            CubeState::Inactive => '.',
        };
        write!(f, "{}", c)
    }
}

pub struct CubeGrid {
    cubes: HashMap<Vec3, CubeState>,
}

impl CubeGrid {
    pub fn parse(input: &str) -> Self {
        let mut cubes = HashMap::new();

        input.lines()
            .enumerate()
            .for_each(|(n, ln)| {
                ln.chars()
                    .enumerate()
                    .for_each(|(m, c)| {
                        cubes.insert(
                            Vec3::from((m as isize, n as isize, 0 as isize)),
                            match c {
                                '.' => CubeState::Inactive,
                                '#' => CubeState::Active,
                                _ => unreachable!(),
                            }
                        );
                    });
            });
        
        Self { cubes }
    }

    pub fn get(&self, position: &Vec3) -> CubeState {
        match self.cubes.get(position) {
            None => CubeState::Inactive,
            Some(&cs) => cs,
        }
    }

    pub fn next(&self) -> Self {
        let cubes = self.cubes.keys()
            .map(|k| k.neighbors())
            .kmerge()
            .unique()
            .map(|v| {
                let n_active = v.neighbors()
                    .iter()
                    .map(|v| {
                        if let Some(&CubeState::Active) = self.cubes.get(&v) {
                            1
                        } else {
                            0
                        }})
                    .sum::<usize>();
                
                let current_state: CubeState = self.get(&v);
                
                if n_active == 3 {
                    (v, CubeState::Active)
                } else if (n_active == 2) & (current_state == CubeState::Active) {
                    (v, CubeState::Active)
                } else {
                    (v, CubeState::Inactive)
                }
            })
            .collect();

        Self { cubes }
    }

    pub fn active_cubes(&self) -> usize {
        self.cubes.values()
            .filter(|&&cs| cs == CubeState::Active)
            .sum()
    }

    fn span(&self) -> ((isize, isize), (isize, isize), (isize, isize)) {
        let mmx = self.cubes.keys().map(|v| v.x).minmax();
        let mmy = self.cubes.keys().map(|v| v.y).minmax();
        let mmz = self.cubes.keys().map(|v| v.z).minmax();

        (CubeGrid::axis_span(&mmx), 
         CubeGrid::axis_span(&mmy), 
         CubeGrid::axis_span(&mmz))
    }

    fn axis_span(minmax: &itertools::MinMaxResult<isize>) -> (isize, isize) {
        use itertools::MinMaxResult::*;

        match *minmax {
            NoElements => (0, 0),
            OneElement(x) => (x, x),
            MinMax(x, y) => (x, y)
        }
    }
}

impl fmt::Display for CubeGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((minx, maxx), (miny, maxy), (minz, maxz)) = self.span();

        (minz..=maxz)
            .for_each(|z| {
                writeln!(f, "z={}\n", z);

                (miny..=maxy)
                    .for_each(|y| {
                        (minx..=maxx)
                            .for_each(|x| {
                                match self.cubes.get(&Vec3::from((x, y, z))) {
                                    None => write!(f, "."),
                                    Some(c) => write!(f, "{}", c)
                                };
                            });
                            write!(f, "\n");
                    });
                
                write!(f, "\n");
            });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_neighbors() {
        let v = Vec3::from((1, 2, 3));
        let mut expected_neighbors = vec![
                (0, 2, 3),
                (2, 2, 3),
                (1, 1, 3),
                (1, 3, 3),
                (1, 2, 2),
                (1, 2, 4)
            ]
            .into_iter()
            .map(|t| Vec3::from(t))
            .collect::<Vec<_>>();
        expected_neighbors.sort();
        
        let mut ns = v.neighbors();
        ns.sort();

        assert_eq!(ns, expected_neighbors);
    }

    #[test]
    fn test_cubegrid_parse() {
        let text = ".#.\n..#\n###";
        let posns = vec![(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)];

        let grid = CubeGrid::parse(text);
        for p in posns {
            assert_eq!(grid.cubes.get(&Vec3::from(p)), Some(&CubeState::Active));
        }
    }
}