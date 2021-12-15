pub trait AocSolution {
    fn part1(&self) -> String;
    fn part2(&self) -> String;

    fn with_input_path(input_path: &str) -> Self;
}
