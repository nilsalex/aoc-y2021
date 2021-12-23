use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
struct Amphi {
    amphi_type: AmphiType,
    initial_pos: (u8, u8),
    state: AmphiState,
}

impl Amphi {
    fn pos(&self) -> (u8, u8) {
        match self.state {
            AmphiState::Initial(c, r) => (c, r),
            AmphiState::MoveOutside(c, r) => (c, r),
            AmphiState::RestOutside(c, r) => (c, r),
            AmphiState::MoveInside(c, r) => (c, r),
            AmphiState::Final(c, r) => (c, r),
        }
    }
    fn forbidden_rest(&self) -> bool {
        match self.state {
            AmphiState::RestOutside(c, _) => matches!(c, 2 | 4 | 6 | 8),
            _ => false,
        }
    }
    fn next(&self) -> Vec<(Self, usize)> {
        let e = self.amphi_type.energy();
        match self.state {
            AmphiState::Initial(c, r) => vec![(
                Self {
                    state: AmphiState::MoveOutside(c, r + 1),
                    ..*self
                },
                e,
            )],
            AmphiState::MoveOutside(c, r) => {
                if r < 2 {
                    vec![(
                        Self {
                            state: AmphiState::MoveOutside(c, r + 1),
                            ..*self
                        },
                        e,
                    )]
                } else {
                    let mut next = match c.cmp(&self.initial_pos.0) {
                        std::cmp::Ordering::Less => {
                            if c > 2 {
                                vec![
                                    (
                                        Self {
                                            state: AmphiState::MoveOutside(c - 1, r),
                                            ..*self
                                        },
                                        e,
                                    ),
                                    (
                                        Self {
                                            state: AmphiState::RestOutside(c - 1, r),
                                            ..*self
                                        },
                                        e,
                                    ),
                                ]
                            } else {
                                vec![(
                                    Self {
                                        state: AmphiState::RestOutside(c - 1, r),
                                        ..*self
                                    },
                                    e,
                                )]
                            }
                        }
                        std::cmp::Ordering::Equal => vec![
                            (
                                Self {
                                    state: AmphiState::MoveOutside(c - 1, r),
                                    ..*self
                                },
                                e,
                            ),
                            (
                                Self {
                                    state: AmphiState::RestOutside(c - 1, r),
                                    ..*self
                                },
                                e,
                            ),
                            (
                                Self {
                                    state: AmphiState::MoveOutside(c + 1, r),
                                    ..*self
                                },
                                e,
                            ),
                            (
                                Self {
                                    state: AmphiState::RestOutside(c + 1, r),
                                    ..*self
                                },
                                e,
                            ),
                        ],
                        std::cmp::Ordering::Greater => {
                            if c < 9 {
                                vec![
                                    (
                                        Self {
                                            state: AmphiState::MoveOutside(c + 1, r),
                                            ..*self
                                        },
                                        e,
                                    ),
                                    (
                                        Self {
                                            state: AmphiState::RestOutside(c + 1, r),
                                            ..*self
                                        },
                                        e,
                                    ),
                                ]
                            } else {
                                vec![(
                                    Self {
                                        state: AmphiState::RestOutside(c + 1, r),
                                        ..*self
                                    },
                                    e,
                                )]
                            }
                        }
                    };
                    if c == self.amphi_type.target_col() {
                        next.push((
                            Self {
                                state: AmphiState::MoveInside(c, r - 1),
                                ..*self
                            },
                            e,
                        ));
                        next.push((
                            Self {
                                state: AmphiState::Final(c, r - 1),
                                ..*self
                            },
                            e,
                        ));
                    }
                    next
                }
            }
            AmphiState::RestOutside(c, r) => vec![(
                Self {
                    state: AmphiState::MoveInside(
                        if c < self.initial_pos.0 { c + 1 } else { c - 1 },
                        r,
                    ),
                    ..*self
                },
                e,
            )],
            AmphiState::MoveInside(c, r) => match c.cmp(&self.amphi_type.target_col()) {
                std::cmp::Ordering::Less => vec![(
                    Self {
                        state: AmphiState::MoveInside(c + 1, r),
                        ..*self
                    },
                    e,
                )],
                std::cmp::Ordering::Equal => {
                    if r == 2 {
                        vec![
                            (
                                Self {
                                    state: AmphiState::MoveInside(c, r - 1),
                                    ..*self
                                },
                                e,
                            ),
                            (
                                Self {
                                    state: AmphiState::Final(c, r - 1),
                                    ..*self
                                },
                                e,
                            ),
                        ]
                    } else {
                        vec![(
                            Self {
                                state: AmphiState::Final(c, r - 1),
                                ..*self
                            },
                            e,
                        )]
                    }
                }
                std::cmp::Ordering::Greater => vec![(
                    Self {
                        state: AmphiState::MoveInside(c - 1, r),
                        ..*self
                    },
                    e,
                )],
            },
            AmphiState::Final(_, _) => vec![],
        }
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum AmphiType {
    A,
    B,
    C,
    D,
}

impl AmphiType {
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
    fn char(&self) -> char {
        match self {
            AmphiType::A => 'A',
            AmphiType::B => 'B',
            AmphiType::C => 'C',
            AmphiType::D => 'D',
        }
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum AmphiState {
    Initial(u8, u8),
    MoveOutside(u8, u8),
    RestOutside(u8, u8),
    MoveInside(u8, u8),
    Final(u8, u8),
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct State {
    energy: usize,
    amphis: [Amphi; 8],
    path: Vec<[Amphi; 8]>,
}

fn amphis_to_string(amphis: &[Amphi; 8]) -> String {
    let mut template = "#############\n#...........#\n###.#.#.#.###\n  #.#.#.#.#\n  #########  "
        .chars()
        .collect::<Vec<char>>();

    for amphi in amphis {
        let (c, r) = amphi.pos();
        let character = amphi.amphi_type.char();

        template[(14 * (3 - r) + c + 1) as usize] = character;
    }

    String::from_iter(template.iter())
}

fn all_amphis_are_final(amphis: &[Amphi; 8]) -> bool {
    amphis
        .iter()
        .all(|a| matches!(a.state, AmphiState::Final(_, _)))
}

fn amphis_overlap(amphis: &[Amphi; 8]) -> bool {
    let positions: HashSet<(u8, u8)> = HashSet::from_iter(amphis.iter().map(Amphi::pos));
    positions.len() < 8
}

fn forbidden_rest(amphis: &[Amphi; 8]) -> bool {
    amphis.iter().any(Amphi::forbidden_rest)
}

fn get_next_amphis(amphis: &[Amphi; 8]) -> Vec<([Amphi; 8], usize)> {
    for (i, amphi) in amphis.iter().enumerate() {
        if matches!(
            amphi.state,
            AmphiState::MoveOutside(_, _) | AmphiState::MoveInside(_, _)
        ) {
            return amphi
                .next()
                .iter()
                .map(|(a, e)| {
                    let mut next_amphis = *amphis;
                    next_amphis[i] = *a;
                    (next_amphis, *e)
                })
                .filter(|(amphis, _)| !amphis_overlap(amphis) && !forbidden_rest(amphis))
                .collect();
        }
    }

    let mut result = vec![];

    for (i, amphi) in amphis.iter().enumerate() {
        result.append(
            &mut amphi
                .next()
                .iter()
                .map(|(a, e)| {
                    let mut next_amphis = *amphis;
                    next_amphis[i] = *a;
                    (next_amphis, *e)
                })
                .filter(|(amphis, _)| !amphis_overlap(amphis) && !forbidden_rest(amphis))
                .collect(),
        );
    }

    result
}

fn part1() -> usize {
    let start = [
        Amphi {
            amphi_type: AmphiType::A,
            initial_pos: (4, 0),
            state: AmphiState::Initial(4, 0),
        },
        Amphi {
            amphi_type: AmphiType::A,
            initial_pos: (6, 0),
            state: AmphiState::Initial(6, 0),
        },
        Amphi {
            amphi_type: AmphiType::B,
            initial_pos: (2, 1),
            state: AmphiState::Initial(2, 1),
        },
        Amphi {
            amphi_type: AmphiType::B,
            initial_pos: (4, 1),
            state: AmphiState::Initial(4, 1),
        },
        Amphi {
            amphi_type: AmphiType::C,
            initial_pos: (2, 0),
            state: AmphiState::Initial(2, 0),
        },
        Amphi {
            amphi_type: AmphiType::C,
            initial_pos: (8, 0),
            state: AmphiState::Initial(8, 0),
        },
        Amphi {
            amphi_type: AmphiType::D,
            initial_pos: (6, 1),
            state: AmphiState::Initial(6, 1),
        },
        Amphi {
            amphi_type: AmphiType::D,
            initial_pos: (8, 1),
            state: AmphiState::Initial(8, 1),
        },
    ];

    // initialize dist and heap
    let mut dist: HashMap<[Amphi; 8], usize> = HashMap::new();

    let mut heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();

    // set up dist and heap for starting point
    dist.insert(start, 0);

    heap.push(Reverse(State {
        energy: 0,
        amphis: start,
        path: vec![start],
    }));

    while let Some(Reverse(State {
        energy,
        amphis,
        path,
    })) = heap.pop()
    {
        if all_amphis_are_final(&amphis) {
            path.iter()
                .for_each(|p| println!("{}\n", amphis_to_string(p)));
            return energy;
        }

        if energy > *dist.get(&amphis).unwrap_or(&usize::MAX) {
            continue;
        }

        for (next_amphis, delta_energy) in get_next_amphis(&amphis) {
            let mut next_path = path.clone();
            next_path.push(next_amphis);
            let next = State {
                energy: energy + delta_energy,
                amphis: next_amphis,
                path: next_path,
            };

            if next.energy < *dist.get(&next.amphis).unwrap_or(&usize::MAX) {
                heap.push(Reverse(next.clone()));
                dist.insert((&next).amphis, (&next).energy);
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
