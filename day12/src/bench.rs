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

    #[test]
    fn test_part1() {
        assert!(Solution::with_input_path("input.txt").part1() == 3450)
    }

    #[test]
    fn test_part2() {
        assert!(Solution::with_input_path("input.txt").part2() == 96528)
    }
}
