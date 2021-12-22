use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Cube {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Cube {
    fn in_bounds(&self, size: i32) -> bool {
        self.x_min >= -size
            && self.x_max <= size
            && self.y_min >= -size
            && self.y_max <= size
            && self.z_min >= -size
            && self.z_max <= size
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min.max(other.y_min);
        let y_max = self.y_max.min(other.y_max);
        let z_min = self.z_min.max(other.z_min);
        let z_max = self.z_max.min(other.z_max);

        if x_min <= x_max && y_min <= y_max && z_min <= z_max {
            Some(Cube {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            })
        } else {
            None
        }
    }
    fn difference(&self, other: &Self) -> Vec<Self> {
        if let Some(int) = self.intersection(other) {
            let mut new_cubes: Vec<Self> = vec![];
            for x_bounds in [
                (self.x_min, int.x_min - 1),
                (int.x_min, int.x_max),
                (int.x_max + 1, self.x_max),
            ] {
                for y_bounds in [
                    (self.y_min, int.y_min - 1),
                    (int.y_min, int.y_max),
                    (int.y_max + 1, self.y_max),
                ] {
                    for z_bounds in [
                        (self.z_min, int.z_min - 1),
                        (int.z_min, int.z_max),
                        (int.z_max + 1, self.z_max),
                    ] {
                        let new_cube = Cube {
                            x_min: x_bounds.0,
                            x_max: x_bounds.1,
                            y_min: y_bounds.0,
                            y_max: y_bounds.1,
                            z_min: z_bounds.0,
                            z_max: z_bounds.1,
                        };
                        if new_cube.is_valid() && new_cube != int {
                            new_cubes.push(new_cube);
                        }
                    }
                }
            }
            new_cubes
        } else {
            vec![*self]
        }
    }
    fn size(&self) -> usize {
        (self.x_max - self.x_min + 1) as usize
            * (self.y_max - self.y_min + 1) as usize
            * (self.z_max - self.z_min + 1) as usize
    }
    fn is_valid(&self) -> bool {
        self.x_min <= self.x_max && self.y_min <= self.y_max && self.z_min <= self.z_max
    }
}

fn parse_bounds(str: &str) -> (i32, i32) {
    let (min, max) = str[2..].split_once("..").unwrap();
    (min.parse().unwrap(), max.parse().unwrap())
}

fn parse_step(str: &str) -> (Cube, bool) {
    let (flag, bounds) = str.split_once(' ').unwrap();

    let on = flag == "on";

    let parsed_bounds: Vec<(i32, i32)> = bounds.split(',').map(parse_bounds).collect();

    (
        Cube {
            x_min: parsed_bounds[0].0,
            x_max: parsed_bounds[0].1,
            y_min: parsed_bounds[1].0,
            y_max: parsed_bounds[1].1,
            z_min: parsed_bounds[2].0,
            z_max: parsed_bounds[2].1,
        },
        on,
    )
}

fn part1() -> usize {
    let file = File::open("day22/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let steps: Vec<(Cube, bool)> = lines.map(|l| parse_step(&l)).collect();

    let mut cubes: Vec<Cube> = vec![];

    for (cube, on) in steps {
        if !cube.in_bounds(50) {
            continue;
        }

        let mut new_cubes: Vec<Cube> = vec![];

        for other_cube in cubes {
            let mut diff = other_cube.difference(&cube);
            new_cubes.append(&mut diff);
        }

        if on {
            new_cubes.push(cube);
        }

        cubes = new_cubes;
    }

    cubes.iter().map(Cube::size).sum()
}

fn part2() -> usize {
    let file = File::open("day22/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let steps: Vec<(Cube, bool)> = lines.map(|l| parse_step(&l)).collect();

    let mut cubes: Vec<Cube> = vec![];

    for (cube, on) in steps {
        let mut new_cubes: Vec<Cube> = vec![];

        for other_cube in cubes {
            let mut diff = other_cube.difference(&cube);
            new_cubes.append(&mut diff);
        }

        if on {
            new_cubes.push(cube);
        }

        cubes = new_cubes;
    }

    cubes.iter().map(Cube::size).sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
