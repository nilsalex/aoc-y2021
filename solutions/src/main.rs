use std::fmt::Display;

use utils::AocSolution;

fn print_solution<T, A, B>(solution: T, day: u32)
where
    A: Display,
    B: Display,
    T: AocSolution<A, B>,
{
    println!(
        "# Day {}\n{}\n{}\n",
        day,
        solution.part1(),
        solution.part2()
    );
}

fn main() {
    print_solution(day01::Solution::with_input_path("day01/input.txt"), 1);
    print_solution(day02::Solution::with_input_path("day02/input.txt"), 2);
    print_solution(day03::Solution::with_input_path("day03/input.txt"), 3);
    print_solution(day04::Solution::with_input_path("day04/input.txt"), 4);
    print_solution(day05::Solution::with_input_path("day05/input.txt"), 5);
    print_solution(day06::Solution::with_input_path("day06/input.txt"), 6);
    print_solution(day07::Solution::with_input_path("day07/input.txt"), 7);
    print_solution(day08::Solution::with_input_path("day08/input.txt"), 8);
    print_solution(day09::Solution::with_input_path("day09/input.txt"), 9);
    print_solution(day10::Solution::with_input_path("day10/input.txt"), 10);
    print_solution(day11::Solution::with_input_path("day11/input.txt"), 11);
    print_solution(day12::Solution::with_input_path("day12/input.txt"), 12);
    print_solution(day13::Solution::with_input_path("day13/input.txt"), 13);
    // print_solution(day14::Solution::with_input_path("day14/input.txt"), 14);
    // print_solution(day15::Solution::with_input_path("day15/input.txt"), 15);
    print_solution(day16::Solution::with_input_path("day16/input.txt"), 16);
}
