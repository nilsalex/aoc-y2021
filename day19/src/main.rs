use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

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
) -> Option<Vec<(i32, i32, i32)>> {
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
                                return Some(
                                    probe2_
                                        .iter()
                                        .map(|(x__, y__, z__)| {
                                            (x__ + diff.0, y__ + diff.1, z__ + diff.2)
                                        })
                                        .collect(),
                                );
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

fn part1() -> usize {
    let file = File::open("day19/input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().flatten();

    let mut maps: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut cur_map: Vec<(i32, i32, i32)> = vec![];

    while let Some(line) = lines.next() {
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

    let mut result_maps: Vec<Vec<(i32, i32, i32)>> = vec![maps[0].clone()];
    let mut remaining_maps: Vec<Vec<(i32, i32, i32)>> = maps.iter().skip(1).cloned().collect();

    while !remaining_maps.is_empty() {
        let mut new_result_maps: Vec<Vec<(i32, i32, i32)>> = vec![];
        let mut new_remaining_maps: Vec<Vec<(i32, i32, i32)>> = vec![];
        println!("{}", remaining_maps.len());

        'outer: for map2 in remaining_maps.iter() {
            for map1 in result_maps.iter() {
                if let Some(map2_) = transform_to(map1, map2) {
                    println!("overlap!");
                    new_result_maps.push(map2_);
                    continue 'outer;
                }
            }
            new_remaining_maps.push(map2.clone());
        }

        result_maps.append(&mut new_result_maps);
        remaining_maps = new_remaining_maps;
    }

    let mut result_set: HashSet<(i32, i32, i32)> = HashSet::new();

    for map in result_maps.iter() {
        for pos in map.iter() {
            result_set.insert(*pos);
        }
    }

    println!("{}", result_maps.len());

    result_set.len()
}

fn part2() -> usize {
    let file = File::open("day19/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    0
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
