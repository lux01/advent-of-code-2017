use std::ops::Index;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KnotHasher {
    list: Vec<u32>,
    offset: usize,
    skip_size: usize,
}

impl KnotHasher {
    pub fn new(max: u32) -> KnotHasher {
        KnotHasher {
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

    pub fn sparse_hash(lengths: &[usize]) -> Vec<u32> {
        KnotHasher::new(256).all_rounds(lengths).list
    }

    pub fn hash(lengths: &[usize]) -> KnotHash {
        let mut dense_hash_bytes = [0; 16];
        let sparse_hash = KnotHasher::sparse_hash(lengths);
        let mut blocks = sparse_hash.chunks(16).map(|block| {
            block[1..].iter().fold(block[0], |acc, &val| acc ^ val)
        });

        for i in 0..16 {
            dense_hash_bytes[i] = blocks.next().unwrap() as u8;
        }

        KnotHash { bytes: dense_hash_bytes }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct KnotHash {
    bytes: [u8; 16],
}

impl KnotHash {
    pub fn hash_str(input: &str) -> KnotHash {
        let lengths = {
            let mut input_vec: Vec<_> = input
                .trim()
                .as_bytes()
                .iter()
                .map(|&byte| byte as usize)
                .collect();
            input_vec.append(&mut vec![17, 31, 73, 47, 23]);
            input_vec
        };

        KnotHasher::hash(&lengths)
    }


    pub fn hash_list_of_lengths(input: &str) -> KnotHash {
        let lengths: Vec<_> = input
            .trim()
            .split(",")
            .map(|num_str| num_str.parse().unwrap())
            .collect();

        KnotHasher::hash(&lengths)
    }

    pub fn to_str(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
            self.bytes[6],
            self.bytes[7],
            self.bytes[8],
            self.bytes[9],
            self.bytes[10],
            self.bytes[11],
            self.bytes[12],
            self.bytes[13],
            self.bytes[14],
            self.bytes[15]
        )
    }
}

impl PartialEq<str> for KnotHash {
    fn eq(&self, other: &str) -> bool {
        self.to_str() == other
    }
}

impl<'a> PartialEq<KnotHash> for &'a str {
    fn eq(&self, other: &KnotHash) -> bool {
        other.to_str() == *self
    }
}

impl Index<u8> for KnotHash {
    type Output = u8;

    fn index(&self, index: u8) -> &u8 {
        self.bytes.index(index as usize)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_list_rotate() {
        let mut hash = KnotHasher::new(5);

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
        let output_hash = KnotHasher::new(5).check_round(&[3, 4, 1, 5]);
        assert_eq!(12, output_hash);
    }

    #[test]
    fn hash_str() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", KnotHash::hash_str(""));
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            KnotHash::hash_str("AoC 2017")
        );
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            KnotHash::hash_str("1,2,3")
        );
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            KnotHash::hash_str("1,2,4")
        );
    }
}