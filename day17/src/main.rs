struct State {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl State {
    fn initial(vx: i32, vy: i32) -> Self {
        State { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vx = match self.vx.cmp(&0) {
            std::cmp::Ordering::Less => self.vx + 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.vx - 1,
        };
        self.vy -= 1;
    }

    fn out_of_bounds(&self, x_max: i32, y_min: i32) -> bool {
        self.x > x_max || self.y < y_min
    }

    fn in_target(&self, x_target: (i32, i32), y_target: (i32, i32)) -> bool {
        self.x >= x_target.0 && self.x <= x_target.1 && self.y >= y_target.0 && self.y <= y_target.1
    }
}

fn part1() -> i32 {
    let x_target: (i32, i32) = (85, 145);
    let y_target: (i32, i32) = (-163, -108);

    let mut result: i32 = i32::MIN;

    for vx in 0..=2000 {
        for vy in -2000..=2000 {
            let mut state = State::initial(vx, vy);
            let mut y_max: i32 = i32::MIN;

            loop {
                y_max = std::cmp::max(y_max, state.y);

                if state.out_of_bounds(x_target.1, y_target.0) {
                    break;
                }

                if state.in_target(x_target, y_target) {
                    result = std::cmp::max(y_max, result);
                    break;
                }

                state.step()
            }
        }
    }

    result
}

fn part2() -> usize {
    let x_target: (i32, i32) = (85, 145);
    let y_target: (i32, i32) = (-163, -108);

    let mut result: usize = 0;

    for vx in 0..=2000 {
        for vy in -2000..=2000 {
            let mut state = State::initial(vx, vy);

            loop {
                if state.out_of_bounds(x_target.1, y_target.0) {
                    break;
                }

                if state.in_target(x_target, y_target) {
                    result += 1;
                    break;
                }

                state.step()
            }
        }
    }

    result
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
