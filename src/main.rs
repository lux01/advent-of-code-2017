extern crate advent_of_code_2017;

use advent_of_code_2017::*;

macro_rules! day {
    ($day:ident => $path:expr) => ({
        let file_contents = file_as_string($path);
        $day::from_str(&file_contents).run();
    })
}

pub fn main() {
    day!(Day01 => "input/day_01.txt");
    day!(Day02 => "input/day_02.txt");
    day!(Day03 => "input/day_03.txt");
    day!(Day04 => "input/day_04.txt");
    day!(Day05 => "input/day_05.txt");
    day!(Day06 => "input/day_06.txt");
    day!(Day07 => "input/day_07.txt");
}