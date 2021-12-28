fn main() {
    macro_rules! day {
        ($day:ident,$part:ident) => {
            println!("{:?}", aoc2021::$day::$part(aoc2021::$day::INPUT));
        };
    }

    day!(day01, part1);
}
