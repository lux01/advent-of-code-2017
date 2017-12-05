use super::Day;

use std::u32;
use std::cmp;

/// [Day 2](https://adventofcode.com/2017/day/2). Calculate the checksum of a spreadsheet.
pub struct Day02 {
    rows: Vec<Vec<u32>>,
}

impl Day02 {
    fn row_map_reduce(&self, mapfn: fn(&[u32]) -> u32) -> isize {
        self.rows.iter().map(|row| mapfn(row)).sum::<u32>() as isize
    }
}

impl Day for Day02 {
    const NUM: u32 = 2;

    fn from_str(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|word| word.parse().unwrap())
                    .collect()
            })
            .collect();

        Day02 { rows }
    }

    fn part_1(&self) -> isize {
        self.row_map_reduce(|row| {
            let (max, min) = row.iter().fold((0, u32::MAX), |(max, min), val| {
                (cmp::max(max, *val), cmp::min(min, *val))
            });

            max - min
        })
    }

    fn part_2(&self) -> isize {
        self.row_map_reduce(|row| {
            for i in 0..row.len() {
                for j in (i + 1)..row.len() {
                    if row[i] % row[j] == 0 {
                        return row[i] / row[j];
                    } else if row[j] % row[i] == 0 {
                        return row[j] / row[i];
                    }
                }
            }
            unreachable!()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &'static str = "5 1 9 5
7 5 3
2 4 6 8";

    const TEST_INPUT_2: &'static str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn part_1() {
        assert_eq!(18, Day02::from_str(TEST_INPUT_1).part_1());
    }

    #[test]
    fn part_2() {
        assert_eq!(9, Day02::from_str(TEST_INPUT_2).part_2());
    }
}