use super::Day;

use std::collections::HashSet;

pub struct Day04 {
    passphrases: Vec<String>,
}

impl Day04 {
    pub fn is_passphrase_valid(passphrase: &str) -> bool {
        let mut set: HashSet<String> = HashSet::new();

        passphrase.split_whitespace().all(|word| {
            set.insert(word.to_owned())
        })
    }

    pub fn is_passphrase_even_more_valid(passphrase: &str) -> bool {
        let mut set: HashSet<String> = HashSet::new();

        passphrase.split_whitespace().all(|word| {
            let mut word_chars = word.chars().collect::<Vec<_>>();
            word_chars.sort();
            set.insert(word_chars.into_iter().collect())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_passphrase_valid_1() {
        assert_eq!(true, Day04::is_passphrase_valid("aa bb cc dd ee"));
        assert_eq!(false, Day04::is_passphrase_valid("aa bb cc dd aa"));
        assert_eq!(false, Day04::is_passphrase_valid("aa bb cc dd aa aaa"));
    }

    #[test]
    fn is_passphrase_even_more_valid() {
        assert_eq!(true, Day04::is_passphrase_even_more_valid("abcde fghij"));
        assert_eq!(
            false,
            Day04::is_passphrase_even_more_valid("abcde xyz ecdab")
        );
        assert_eq!(
            true,
            Day04::is_passphrase_even_more_valid("a ab abc abd abf abj")
        );
        assert_eq!(
            true,
            Day04::is_passphrase_even_more_valid("iiii oiii ooii oooi oooo")
        );
        assert_eq!(
            false,
            Day04::is_passphrase_even_more_valid("oiii ioii iioi iiio")
        );
    }
}

impl Day for Day04 {
    const NUM: u32 = 4;

    fn from_str(input: &str) -> Day04 {
        let passphrases = input.lines().map(|line| line.to_owned()).collect();

        Day04 { passphrases }
    }

    fn part_1(&self) -> isize {
        self.passphrases
            .iter()
            .filter(|phrase| Day04::is_passphrase_valid(phrase))
            .count() as isize
    }

    fn part_2(&self) -> isize {
        self.passphrases
            .iter()
            .filter(|phrase| Day04::is_passphrase_even_more_valid(phrase))
            .count() as isize
    }
}