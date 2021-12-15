#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use day01::lib::Day01;
    use day02::lib::Day02;
    use utils::AocSolution;

    #[bench]
    fn bench_day01_part1(b: &mut Bencher) {
        b.iter(|| Day01::with_input_path("../day01/input.txt").part1())
    }

    #[bench]
    fn bench_day01_part2(b: &mut Bencher) {
        b.iter(|| Day01::with_input_path("../day01/input.txt").part2())
    }

    #[bench]
    fn bench_day02_part1(b: &mut Bencher) {
        b.iter(|| Day02::with_input_path("../day02/input.txt").part1())
    }

    #[bench]
    fn bench_day02_part2(b: &mut Bencher) {
        b.iter(|| Day02::with_input_path("../day02/input.txt").part2())
    }
}
