use day01::lib::Day01;
use utils::AocSolution;

fn main() {
    let solution = Day01::with_input_path("day01/input.txt");
    println!("{}", solution.part1());
    println!("{}", solution.part2());
}
