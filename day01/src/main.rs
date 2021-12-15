use day01::lib::Day01;
use utils::AocSolution;

fn main() {
    let day01 = Day01::with_input_path("day01/input.txt");
    println!("{}", day01.part1());
    println!("{}", day01.part2());
}
