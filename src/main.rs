extern crate advent_of_code_2017;
extern crate clap;

use clap::{Arg, App};

use advent_of_code_2017::*;

macro_rules! day {
    ($day:ident => $path:expr) => ({
        let file_contents = file_as_string($path);
        $day::from_str(&file_contents).run();
    })
}

pub fn main() {
    let matches = App::new("Advent of Code 2017")
        .version("0.1.0")
        .author("William Woodhead <william.woodhead@lux01.co.uk>")
        .about(
            "Calculates the solutions to an implemented day of the Advent of Code 2017",
        )
        .arg(
            Arg::with_name("DAY")
                .help("The number of the day to run")
                .required(true)
                .index(1),
        )
        .get_matches();

    match matches.value_of("DAY").unwrap() {
        "1" | "01" => day!(Day01 => "input/day_01.txt"),
        "2" | "02" => day!(Day02 => "input/day_02.txt"),
        "3" | "03" => day!(Day03 => "input/day_03.txt"),
        "4" | "04" => day!(Day04 => "input/day_04.txt"),
        "5" | "05" => day!(Day05 => "input/day_05.txt"),
        "6" | "06" => day!(Day06 => "input/day_06.txt"),
        "7" | "07" => day!(Day07 => "input/day_07.txt"),
        "8" | "08" => day!(Day08 => "input/day_08.txt"),
        "9" | "09" => day!(Day09 => "input/day_09.txt"),
        "10" => day!(Day10 => "input/day_10.txt"),
        "11" => day!(Day11 => "input/day_11.txt"),
        "12" => day!(Day12 => "input/day_12.txt"),
        "13" => day!(Day13 => "input/day_13.txt"),
        "15" => day!(Day15 => "input/day_15.txt"),
        "16" => day!(Day16 => "input/day_16.txt"),
        _ => println!("Unknown day"),
    }
}