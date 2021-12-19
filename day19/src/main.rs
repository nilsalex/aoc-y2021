use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

struct Result {
    scanners: Vec<(i32, i32, i32)>,
    beacons: HashSet<(i32, i32, i32)>,
}

fn rotate_x(pos: &mut (i32, i32, i32)) {
    *pos = (pos.0, pos.2, -pos.1)
}

fn rotate_y(pos: &mut (i32, i32, i32)) {
    *pos = (-pos.2, pos.1, pos.0)
}

fn rotate_z(pos: &mut (i32, i32, i32)) {
    *pos = (-pos.1, pos.0, pos.2)
}

fn rotate(pos: &mut (i32, i32, i32), rots: &mut (u8, u8, u8)) {
    if rots.0 > 0 {
        rotate_x(pos);
        rots.0 -= 1;
        rotate(pos, rots);
    } else if rots.1 > 0 {
        rotate_y(pos);
        rots.1 -= 1;
        rotate(pos, rots);
    } else if rots.2 > 0 {
        rotate_z(pos);
        rots.2 -= 1;
        rotate(pos, rots);
    }
}

fn transform_to(
    probe1: &[(i32, i32, i32)],
    probe2: &[(i32, i32, i32)],
) -> Option<((i32, i32, i32), Vec<(i32, i32, i32)>)> {
    for rotx in 0_u8..=3 {
        for roty in 0_u8..=3 {
            for rotz in 0_u8..=3 {
                let transformed = probe2.iter().map(|pos| {
                    let mut new_pos = *pos;
                    let mut rots = (rotx, roty, rotz);
                    rotate(&mut new_pos, &mut rots);
                    new_pos
                });
                let probe2_: Vec<(i32, i32, i32)> = transformed.collect();

                let mut diffs: HashMap<(i32, i32, i32), usize> = HashMap::new();

                for (x, y, z) in probe1.iter() {
                    for (x_, y_, z_) in &probe2_ {
                        let diff = (x - x_, y - y_, z - z_);
                        if let Some(count) = diffs.get_mut(&diff) {
                            *count += 1;
                            if *count == 12 {
                                return Some((
                                    diff,
                                    probe2_
                                        .iter()
                                        .map(|(x__, y__, z__)| {
                                            (x__ + diff.0, y__ + diff.1, z__ + diff.2)
                                        })
                                        .collect(),
                                ));
                            }
                        } else {
                            diffs.insert(diff, 1);
                        }
                    }
                }
            }
        }
    }

    None
}

fn transform_maps(maps: &[Vec<(i32, i32, i32)>]) -> Result {
    let mut beacons = HashSet::from_iter(maps[0].clone());
    let mut scanners = vec![(0, 0, 0)];
    let mut to_check: Vec<Vec<(i32, i32, i32)>> = vec![maps[0].clone()];
    let mut remaining: Vec<usize> = Vec::from_iter(1..maps.len());

    while !remaining.is_empty() {
        let mut new_remaining: Vec<usize> = vec![];
        let mut new_to_check: Vec<Vec<(i32, i32, i32)>> = vec![];

        'outer: for i in remaining.clone() {
            for j in to_check.iter() {
                if let Some((diff, points)) = transform_to(j, &maps[i]) {
                    new_to_check.push(points.clone());
                    beacons.extend(points.iter());
                    scanners.push(diff);
                    continue 'outer;
                }
            }
            new_remaining.push(i);
        }

        to_check = new_to_check;
        remaining = new_remaining;
    }

    Result { scanners, beacons }
}

fn part1() -> usize {
    let file = File::open("day19/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut maps: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut cur_map: Vec<(i32, i32, i32)> = vec![];

    for line in lines {
        if line.is_empty() {
            maps.push(cur_map.clone());
            continue;
        }

        if line.starts_with("---") {
            cur_map = vec![];
            continue;
        }

        let pos = match &line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[0..]
        {
            &[x_, y_, z_, ..] => (x_, y_, z_),
            _ => unreachable!(),
        };

        cur_map.push(pos);
    }

    maps.push(cur_map.clone());

    transform_maps(&maps).beacons.len()
}

fn part2() -> usize {
    let file = File::open("day19/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let mut maps: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut cur_map: Vec<(i32, i32, i32)> = vec![];

    for line in lines {
        if line.is_empty() {
            maps.push(cur_map.clone());
            continue;
        }

        if line.starts_with("---") {
            cur_map = vec![];
            continue;
        }

        let pos = match &line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[0..]
        {
            &[x_, y_, z_, ..] => (x_, y_, z_),
            _ => unreachable!(),
        };

        cur_map.push(pos);
    }

    maps.push(cur_map.clone());

    let Result {
        scanners,
        beacons: _,
    } = transform_maps(&maps);

    let mut max_diff: usize = 0;

    for (x1, y1, z1) in scanners.iter() {
        for (x2, y2, z2) in scanners.iter() {
            max_diff = std::cmp::max(
                max_diff,
                ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize,
            );
        }
    }

    max_diff
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
