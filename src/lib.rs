pub mod day_01;
pub mod day_02;
pub mod day_03;

pub use day_01::Day01;
pub use day_02::Day02;
pub use day_03::Day03;

use std::fs::File;
use std::path::Path;
use std::io::Read;

pub trait Day {
    const NUM: u32;

    fn from_str(input: &str) -> Self;

    fn part_1(&self) -> isize;
    fn part_2(&self) -> isize;

    fn run(&self) {
        println!("Day {}", Self::NUM);
        println!("=======================================");
        println!("Part 1: {}", self.part_1());
        println!("Part 2: {}", self.part_2());
        println!("");
    }
}

pub fn input_file<P: AsRef<Path>>(path: P) -> String {
    let mut buffer = String::new();
    let mut file = File::open(path).expect("Failed to open input file.");
    file.read_to_string(&mut buffer).expect(
        "Failed to read input file.",
    );
    buffer
}
