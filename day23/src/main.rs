#![feature(int_abs_diff)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const DEPTH: u8 = 4;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
struct Amphi {
    amphi_type: AmphiType,
    pos: (u8, u8),
    state: AmphiState,
}

impl Amphi {
    fn parse(str: &str) -> Self {
        let chars = str.chars().collect::<Vec<char>>();

        let amphi_type = match chars[0] {
            'A' => AmphiType::A,
            'B' => AmphiType::B,
            'C' => AmphiType::C,
            'D' => AmphiType::D,
            _ => unreachable!(),
        };
        let c = chars[2].to_digit(10).unwrap() as u8;
        let r = chars[3].to_digit(10).unwrap() as u8;
        let pos = (c, r);

        match chars[1] {
            'I' => Self {
                amphi_type,
                pos,
                state: AmphiState::Initial,
            },
            'F' => Self {
                amphi_type,
                pos,
                state: AmphiState::Final,
            },
            'P' => Self {
                amphi_type,
                pos,
                state: AmphiState::Parked,
            },
            _ => unreachable!(),
        }
    }

    fn next(&self, amphis: &Vec<Self>) -> Vec<(Self, usize)> {
        match self.state {
            AmphiState::Final => vec![],
            AmphiState::Initial => [
                (0, DEPTH),
                (1, DEPTH),
                (3, DEPTH),
                (5, DEPTH),
                (7, DEPTH),
                (9, DEPTH),
                (10, DEPTH),
            ]
            .iter()
            .filter_map(|p| trace_to_parking_pos(self, p, amphis))
            .map(|(new_pos, e)| {
                (
                    Amphi {
                        pos: new_pos,
                        state: AmphiState::Parked,
                        ..*self
                    },
                    e,
                )
            })
            .collect(),
            AmphiState::Parked => trace_to_well(self, &(self.amphi_type.target_col(), 0), amphis)
                .map(|(new_pos, e)| {
                    vec![(
                        Amphi {
                            pos: new_pos,
                            state: AmphiState::Final,
                            ..*self
                        },
                        e,
                    )]
                })
                .unwrap_or(vec![]),
        }
    }
}

fn trace_to_parking_pos(
    amphi: &Amphi,
    end: &(u8, u8),
    amphis: &Amphis,
) -> Option<((u8, u8), usize)> {
    for i in amphi.pos.1 + 1..=end.1 {
        if amphis
            .iter()
            .any(|other_amphi| other_amphi.pos == (amphi.pos.0, i))
        {
            return None;
        }
    }

    if end.0 > amphi.pos.0 {
        for i in amphi.pos.0 + 1..=end.0 {
            if amphis
                .iter()
                .any(|other_amphi| other_amphi.pos == (i, end.1))
            {
                return None;
            }
        }
    } else {
        for i in (end.0..=amphi.pos.0 - 1).rev() {
            if amphis
                .iter()
                .any(|other_amphi| other_amphi.pos == (i, end.1))
            {
                return None;
            }
        }
    }

    Some((
        *end,
        amphi.amphi_type.energy()
            * (end.0.abs_diff(amphi.pos.0) + end.1.abs_diff(amphi.pos.1)) as usize,
    ))
}

fn trace_to_well(amphi: &Amphi, end: &(u8, u8), amphis: &Amphis) -> Option<((u8, u8), usize)> {
    let trace: bool = amphi.amphi_type == AmphiType::C
        && amphi.pos == (5, 2)
        && amphi.state == AmphiState::Parked
        && *end == (6, 0)
        && false;

    if trace {
        println!(
            "tracing amphi to final. Amphi:\n{:?}\n\nAmphis:\n{}\n\n",
            amphi,
            amphis_to_string(&amphis)
        );
    }

    if end.0 > amphi.pos.0 {
        if trace {
            println!("first branch in horizontal traversal!");
        }
        for i in amphi.pos.0 + 1..=end.0 {
            if amphis
                .iter()
                .any(|other_amphi| other_amphi.pos == (i, amphi.pos.1))
            {
                return None;
            }
        }
    } else {
        if trace {
            println!("second branch in horizontal traversal!");
        }
        for i in (end.0..=amphi.pos.0 - 1).rev() {
            if amphis
                .iter()
                .any(|other_amphi| other_amphi.pos == (i, amphi.pos.1))
            {
                return None;
            }
        }
    }

    if trace {
        println!("passed horizontal traversal!");
    }

    for i in (end.1..=amphi.pos.1 - 1).rev() {
        if let Some(other_amphi) = amphis
            .iter()
            .find(|other_amphi| other_amphi.pos == (end.0, i))
        {
            match other_amphi.state {
                AmphiState::Initial => return None,
                AmphiState::Parked => unreachable!(),
                AmphiState::Final => {
                    if trace {
                        println!(
                            "passed vertical traversal. Descending to {:?}",
                            (end.0, i + 1)
                        )
                    }
                    return Some((
                        (end.0, i + 1),
                        amphi.amphi_type.energy()
                            * (end.0.abs_diff(amphi.pos.0) + (i + 1).abs_diff(amphi.pos.1))
                                as usize,
                    ));
                }
            }
        }
    }

    if trace {
        println!("passed vertical traversal. Descending to {:?}", *end)
    }
    Some((
        *end,
        amphi.amphi_type.energy()
            * (end.0.abs_diff(amphi.pos.0) + end.1.abs_diff(amphi.pos.1)) as usize,
    ))
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum AmphiType {
    A,
    B,
    C,
    D,
}

impl AmphiType {
    fn char(&self) -> char {
        match self {
            AmphiType::A => 'A',
            AmphiType::B => 'B',
            AmphiType::C => 'C',
            AmphiType::D => 'D',
        }
    }
    fn target_col(&self) -> u8 {
        match self {
            AmphiType::A => 2,
            AmphiType::B => 4,
            AmphiType::C => 6,
            AmphiType::D => 8,
        }
    }
    fn energy(&self) -> usize {
        match self {
            AmphiType::A => 1,
            AmphiType::B => 10,
            AmphiType::C => 100,
            AmphiType::D => 1000,
        }
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum AmphiState {
    Initial,
    Parked,
    Final,
}

type Amphis = Vec<Amphi>;

#[derive(Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
struct State {
    energy: usize,
    amphis: Amphis,
}

fn amphis_to_string(amphis: &Amphis) -> String {
    let mut chars: Vec<char> = vec![
        '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '\n', '#', '.', '.', '.',
        '.', '.', '.', '.', '.', '.', '.', '.', '#', '\n', '#', '#', '#', '.', '#', '.', '#', '.',
        '#', '.', '#', '#', '#', '\n',
    ];

    for _ in 0..DEPTH - 1 {
        chars.extend([
            ' ', ' ', '#', '.', '#', '.', '#', '.', '#', '.', '#', ' ', ' ', '\n',
        ]);
    }

    chars.extend([
        ' ', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ', ' ', '\n',
    ]);

    for amphi in amphis.iter() {
        chars[15 + 14 * (DEPTH - amphi.pos.1) as usize + amphi.pos.0 as usize] =
            amphi.amphi_type.char();
    }

    String::from_iter(chars.iter())
}

fn get_next_amphis(amphis: &Amphis) -> Vec<(Amphis, usize)> {
    let mut next_amphis = vec![];

    for (i, amphi) in amphis.iter().enumerate() {
        let next_for_this_amphi = amphi.next(&amphis);
        // println!(
        //     "next for amphi\n{:?}\n\n{:?}\n\n{}\n\n----------------------\n",
        //     amphi,
        //     next_for_this_amphi,
        //     amphis_to_string(amphis),
        // );
        next_amphis.append(
            &mut next_for_this_amphi
                .iter()
                .map(|(a, e)| {
                    let mut cloned = amphis.clone();
                    cloned[i] = *a;
                    cloned.sort();
                    (cloned, *e)
                })
                .collect::<Vec<(Amphis, usize)>>(),
        );
    }

    next_amphis
}

fn part1() -> usize {
    // full example
    //
    // let start = [
    //     Amphi::parse("AF20"),
    //     Amphi::parse("DI40"),
    //     Amphi::parse("CF60"),
    //     Amphi::parse("AI80"),
    //     Amphi::parse("DI21"),
    //     Amphi::parse("BI41"),
    //     Amphi::parse("AI61"),
    //     Amphi::parse("CI81"),
    //     Amphi::parse("DI22"),
    //     Amphi::parse("CI42"),
    //     Amphi::parse("BI62"),
    //     Amphi::parse("AI82"),
    //     Amphi::parse("BI23"),
    //     Amphi::parse("CI43"),
    //     Amphi::parse("BI63"),
    //     Amphi::parse("DI83"),
    // ];

    // full real input
    //
    let start = [
        Amphi::parse("CI20"),
        Amphi::parse("AI40"),
        Amphi::parse("AI60"),
        Amphi::parse("CI80"),
        Amphi::parse("DI21"),
        Amphi::parse("BI41"),
        Amphi::parse("AI61"),
        Amphi::parse("CI81"),
        Amphi::parse("DI22"),
        Amphi::parse("CI42"),
        Amphi::parse("BI62"),
        Amphi::parse("AI82"),
        Amphi::parse("BI23"),
        Amphi::parse("BI43"),
        Amphi::parse("DI63"),
        Amphi::parse("DI83"),
    ];

    // short example
    // let start = [
    //     Amphi::parse("AF20"),
    //     Amphi::parse("BI21"),
    //     Amphi::parse("DI40"),
    //     Amphi::parse("CI41"),
    //     Amphi::parse("CF60"),
    //     Amphi::parse("BI61"),
    //     Amphi::parse("AI80"),
    //     Amphi::parse("DI81"),
    // ];

    // short real input
    //
    // let start = [
    //     Amphi::parse("CI20"),
    //     Amphi::parse("AI40"),
    //     Amphi::parse("AI60"),
    //     Amphi::parse("CI80"),
    //     Amphi::parse("BI21"),
    //     Amphi::parse("BI41"),
    //     Amphi::parse("DI61"),
    //     Amphi::parse("DI81"),
    // ];

    // let end = [
    //     Amphi::parse("AF20"),
    //     Amphi::parse("BI21"),
    //     Amphi::parse("DI40"),
    //     Amphi::parse("CF61"),
    //     Amphi::parse("CF60"),
    //     Amphi::parse("BP32"),
    //     Amphi::parse("AI80"),
    //     Amphi::parse("DI81"),
    // ];

    // let mut end_vec = end.to_vec();
    // end_vec.sort();

    // initialize dist and heap
    let mut dist: HashMap<Amphis, usize> = HashMap::new();

    let mut heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();

    // set up dist and heap for starting point
    let mut start_vec = start.to_vec();
    start_vec.sort();
    dist.insert(start_vec.clone(), 0);

    heap.push(Reverse(State {
        energy: 0,
        amphis: start_vec.clone(),
    }));

    while let Some(Reverse(State { energy, amphis })) = heap.pop() {
        if amphis.iter().all(|a| matches!(a.state, AmphiState::Final)) {
            // if amphis == end_vec {
            return energy;
        }

        if energy > *dist.get(&amphis).unwrap_or(&usize::MAX) {
            continue;
        }

        for (next_amphis, delta_energy) in get_next_amphis(&amphis) {
            let next = State {
                energy: energy + delta_energy,
                amphis: next_amphis.clone(),
            };

            if next.energy < *dist.get(&next.amphis).unwrap_or(&usize::MAX) {
                heap.push(Reverse(next.clone()));
                dist.insert((&next).amphis.clone(), (&next).energy);
            }
        }
    }

    panic!();
}

fn part2() -> usize {
    0
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
