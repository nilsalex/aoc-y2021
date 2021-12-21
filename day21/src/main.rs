use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Player {
    One,
    Two,
}

impl Player {
    fn next(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

fn weird_mod(a: usize, b: usize) -> usize {
    ((a - 1) % b) + 1
}

fn quantum(
    player: Player,
    p1: usize,
    p2: usize,
    p1_score: usize,
    p2_score: usize,
    target: usize,
    count: usize,
    probs: &[(usize, usize)],
) -> (usize, usize) {
    probs
        .iter()
        .map(|(roll, next_count)| match player {
            Player::One => {
                let next_p1 = weird_mod(p1 + roll, 10);
                let next_p1_score = p1_score + next_p1;

                if next_p1_score >= target {
                    return (count * *next_count, 0);
                }

                quantum(
                    player.next(),
                    next_p1,
                    p2,
                    next_p1_score,
                    p2_score,
                    target,
                    count * *next_count,
                    probs,
                )
            }
            Player::Two => {
                let next_p2 = weird_mod(p2 + roll, 10);
                let next_p2_score = p2_score + next_p2;

                if next_p2_score >= target {
                    return (0, count * *next_count);
                }

                quantum(
                    player.next(),
                    p1,
                    next_p2,
                    p1_score,
                    next_p2_score,
                    target,
                    count * *next_count,
                    probs,
                )
            }
        })
        .fold((0, 0), |(acc1, acc2), (c1, c2)| (acc1 + c1, acc2 + c2))
}

fn simulate(p1_start: usize, p2_start: usize, target: usize, dice: &[usize]) -> Option<Player> {
    let mut p1 = p1_start;
    let mut p2 = p2_start;

    let mut p1_score: usize = 0;
    let mut p2_score: usize = 0;

    let mut rolls: usize = 0;

    loop {
        p1 = weird_mod(p1 + dice[rolls], 10);
        rolls += 1;

        p1_score += p1;

        if p1_score >= target {
            return Some(Player::One);
        }

        p2 = weird_mod(p2 + dice[rolls], 10);
        rolls += 1;

        p2_score += p2;

        if p2_score >= target {
            return Some(Player::Two);
        }
    }
}

fn part1() -> usize {
    let mut p1: usize = 2;
    let mut p2: usize = 1;

    let mut p1_score: usize = 0;
    let mut p2_score: usize = 0;

    let mut i: usize = 0;

    loop {
        p1 = weird_mod(p1 + 3 * i + 6, 10);
        i += 3;

        p1_score += p1 as usize;

        if p1_score >= 1000 {
            return p2_score * i;
        }

        p2 = weird_mod(p2 + 3 * i + 6, 10);
        i += 3;

        p2_score += p2 as usize;

        if p2_score >= 1000 {
            return p1_score * i;
        }
    }
}

fn part2() -> usize {
    let probs: Vec<(usize, usize)> = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let (p1, p2) = quantum(Player::One, 2, 1, 0, 0, 21, 1, &probs);

    std::cmp::max(p1, p2)
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
