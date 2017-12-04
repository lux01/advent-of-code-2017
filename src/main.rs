extern crate advent_of_code_2017;

use advent_of_code_2017::*;

macro_rules! day {
    ($day:ident => $path:expr) => ({
        let file_contents = input_file($path);
        $day::from_str(&file_contents).run();
    })
}

pub fn main() {
    day!(Day01 => "input/day_01.txt");
    day!(Day02 => "input/day_02.txt");
    day!(Day03 => "input/day_03.txt");
}