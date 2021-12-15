use day02::lib::Day02;
use utils::AocSolution;

fn main() {
    let solution = Day02::with_input_path("day02/input.txt");
    println!("{}", solution.part1());
    println!("{}", solution.part2());
}
