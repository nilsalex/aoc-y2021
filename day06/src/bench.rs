extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Solution;
    use test::Bencher;
    use utils::AocSolution;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| Solution::with_input_path("input.txt").part1())
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| Solution::with_input_path("input.txt").part2())
    }
}
