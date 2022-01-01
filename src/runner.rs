fn main() {
    macro_rules! day {
        ($day:ident,$part:ident) => {
            println!("{:?}", aoc2021::$day::$part(aoc2021::$day::INPUT));
        };
    }

    day!(day01, part1);
    day!(day01, part2);
    day!(day02, part1);
    day!(day02, part2);
    day!(day03, part1);
    day!(day03, part2);
    day!(day04, part1);
    day!(day04, part2);
    day!(day05, part1);
    day!(day05, part2);
    day!(day06, part1);
    day!(day06, part2);
    day!(day07, part1);
    day!(day07, part2);
}
