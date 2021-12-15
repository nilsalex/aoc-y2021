pub trait AocSolution<A: std::fmt::Display, B: std::fmt::Display> {
    fn part1(&self) -> A;
    fn part2(&self) -> B;

    fn with_input_path(input_path: &str) -> Self;
}
