use super::Day;

/// [Day 1](htts://adventofcode.com/2017/day/1). Calculate the solution to a captcha string.
pub struct Day01 {
    numbers: Vec<u32>,
}

impl<'a> Day<'a> for Day01 {
    const NUM: u32 = 1;
    type Output1 = u32;
    type Output2 = u32;

    fn from_str(input: &str) -> Self {
        let numbers = input
            .chars()
            .map(|c| c.to_digit(10).expect("Not a valid digit"))
            .collect::<Vec<u32>>();

        Day01 { numbers }
    }

    fn part_1(&self) -> u32 {
        self.numbers
            .iter()
            .zip(self.numbers.iter().cycle().skip(1))
            .fold(0, |acc, (number, next_number)| if number == next_number {
                acc + number
            } else {
                acc
            })
    }

    fn part_2(&self) -> u32 {
        self.numbers
            .iter()
            .zip(self.numbers.iter().cycle().skip(self.numbers.len() / 2))
            .fold(0, |acc, (number, next_number)| if number == next_number {
                acc + number
            } else {
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1() {
        assert_eq!(3, Day01::from_str("1122").part_1());
        assert_eq!(4, Day01::from_str("1111").part_1());
        assert_eq!(0, Day01::from_str("1234").part_1());
        assert_eq!(9, Day01::from_str("91212129").part_1());
    }

    #[test]
    pub fn part_2() {
        assert_eq!(6, Day01::from_str("1212").part_2());
        assert_eq!(0, Day01::from_str("1221").part_2());
        assert_eq!(4, Day01::from_str("123425").part_2());
        assert_eq!(12, Day01::from_str("123123").part_2());
        assert_eq!(4, Day01::from_str("12131415").part_2());
    }
}