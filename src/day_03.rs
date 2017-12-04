pub use super::Day;

pub struct Day03 {
    number: i32,
}

fn calculate_containing_square(f: i32) -> i32 {
    let n_plus = (1.0 + (4.0 * f as f64 - 3.0).sqrt()) / (4.0);
    n_plus.floor() as i32
}

fn corner_north_east(n: i32) -> i32 {
    4 * n * n - 2 * n + 1
}

fn corner_north_west(n: i32) -> i32 {
    4 * n * n + 1
}

fn corner_south_west(n: i32) -> i32 {
    4 * n * n + 2 * n + 1
}

fn corner_south_east(n: i32) -> i32 {
    4 * n * n + 4 * n + 1
}


impl Day for Day03 {
    const NUM: u32 = 3;

    fn from_str(input: &str) -> Self {
        Day03 { number: input.parse().unwrap() }
    }

    fn part_1(&self) -> isize {
        let n = calculate_containing_square(self.number);
        let ne = corner_north_east(n);
        let nw = corner_north_west(n);
        let sw = corner_south_west(n);
        let se = corner_south_east(n);

        let (x, y) = if ne <= self.number && self.number < nw {
            (n - (self.number - ne), n)
        } else if nw <= self.number && self.number < sw {
            (-n, n - (self.number - nw))
        } else if sw <= self.number && self.number < se {
            (-n + (self.number - sw), -n)
        } else {
            ((self.number - se) - n, n)
        };

        (x + y).abs() as isize
    }

    fn part_2(&self) -> isize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_test_1() {
        assert_eq!(0, Day03::from_str("1").part_1());
    }

    #[test]
    pub fn part_1_test_2() {
        assert_eq!(3, Day03::from_str("12").part_1());
    }
}