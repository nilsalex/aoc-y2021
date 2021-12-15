#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use day01::lib::Day01;
    use utils::AocSolution;

    #[bench]
    fn bench_day01_part1(b: &mut Bencher) {
        b.iter(|| Day01::with_input_path("../day01/input.txt").part1())
    }

    #[bench]
    fn bench_day01_part2(b: &mut Bencher) {
        b.iter(|| Day01::with_input_path("../day01/input.txt").part2())
    }
}
