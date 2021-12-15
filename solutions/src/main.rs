use utils::AocSolution;

fn main() {
    println!(
        "Day 01 part 1: {}",
        day01::Solution::with_input_path("day01/input.txt").part1()
    );
    println!(
        "Day 01 part 2: {}",
        day01::Solution::with_input_path("day01/input.txt").part2()
    );
    println!(
        "Day 02 part 1: {}",
        day02::Solution::with_input_path("day02/input.txt").part1()
    );
    println!(
        "Day 02 part 2: {}",
        day02::Solution::with_input_path("day02/input.txt").part2()
    );
    println!(
        "Day 03 part 1: {}",
        day03::Solution::with_input_path("day03/input.txt").part1()
    );
    println!(
        "Day 03 part 2: {}",
        day03::Solution::with_input_path("day03/input.txt").part2()
    );
}
