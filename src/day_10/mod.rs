use super::Day;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KnotHash {
    list: Vec<u32>,
    offset: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn new(max: u32) -> KnotHash {
        KnotHash {
            list: (0..max).collect(),
            offset: 0,
            skip_size: 0,
        }
    }

    pub fn iterate(&mut self, length: usize) {
        let mut new_list = self.list.clone();
        let list_len = self.list.len();
        for i in 0..length {
            let idx_new = (self.offset + i) % list_len;
            let idx_old = (self.offset + length - i - 1) % list_len;
            new_list[idx_new] = self.list[idx_old];
        }

        self.list = new_list;
        self.offset = (self.offset + length + self.skip_size) % list_len;
        self.skip_size += 1;
    }

    pub fn check_round(&mut self, lengths: &[usize]) -> u32 {
        for length in lengths.iter() {
            self.iterate(*length);
        }
        self.list[0] * self.list[1]
    }

    pub fn all_rounds(mut self, lengths: &[usize]) -> Self {
        for _ in 0..64 {
            for length in lengths.iter() {
                self.iterate(*length);
            }
        }
        self
    }

    pub fn hash(lengths: &[usize]) -> String {
        let sparse_hash = KnotHash::new(256).all_rounds(lengths).list;
        sparse_hash
            .chunks(16)
            .map(|block| {
                block[1..].iter().fold(block[0], |acc, &val| acc ^ val)
            })
            .fold(String::with_capacity(32), |mut acc, block| {
                acc.push_str(&format!("{:02x}", block));
                acc
            })
    }
}

pub struct Day10<'a> {
    input: &'a str,
}

impl<'a> Day10<'a> {
    fn as_nums(input: &str) -> Vec<usize> {
        input
            .trim()
            .split(",")
            .map(|num_str| num_str.parse().unwrap())
            .collect()
    }

    fn as_ascii(input: &str) -> Vec<usize> {
        let mut output: Vec<usize> = input
            .trim()
            .as_bytes()
            .iter()
            .map(|&byte| byte as usize)
            .collect();
        output.append(&mut vec![17, 31, 73, 47, 23]);
        output
    }
}

impl<'a> Day<'a> for Day10<'a> {
    const NUM: u32 = 10;
    type Output1 = u32;
    type Output2 = String;

    fn from_str(input: &'a str) -> Self {
        Day10 { input }
    }

    fn part_1(&self) -> Self::Output1 {
        KnotHash::new(256).check_round(&Day10::as_nums(self.input))
    }

    fn part_2(&self) -> Self::Output2 {
        KnotHash::hash(&Day10::as_ascii(self.input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_list_rotate() {
        let mut hash = KnotHash::new(5);

        hash.iterate(3);
        assert_eq!(vec![2, 1, 0, 3, 4], hash.list);

        hash.iterate(4);
        assert_eq!(vec![4, 3, 0, 1, 2], hash.list);

        hash.iterate(1);
        assert_eq!(vec![4, 3, 0, 1, 2], hash.list);

        hash.iterate(5);
        assert_eq!(vec![3, 4, 2, 1, 0], hash.list);
    }

    #[test]
    fn hash_small_list() {
        let output_hash = KnotHash::new(5).check_round(&[3, 4, 1, 5]);
        assert_eq!(12, output_hash);
    }

    #[test]
    fn day_10_part_2_input_parse_test() {
        let input = "1,2,3";
        assert_eq!(
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23],
            Day10::as_ascii(&input)
        );
    }

    #[test]
    fn dense_hash() {
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272",
            Day10::from_str("").part_2()
        );
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            Day10::from_str("AoC 2017").part_2()
        );
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            Day10::from_str("1,2,3").part_2()
        );
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            Day10::from_str("1,2,4").part_2()
        );


    }
}
