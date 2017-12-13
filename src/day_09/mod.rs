mod parsers;

use super::Day;

#[derive(Debug, PartialEq, Eq)]
pub enum GroupMember {
    Group(u32, Vec<GroupMember>),
    Garbage(String),
}

impl GroupMember {
    pub fn from_str(input: &str) -> GroupMember {
        let (unused, output) = parsers::group(input).unwrap();
        assert_eq!("", unused);
        output.calculate_individual_scores(0)
    }

    pub fn calculate_individual_scores(self, parent_score: u32) -> Self {
        match self {
            GroupMember::Group(_, members) => {
                let new_score = parent_score + 1;
                let updated_members = members
                    .into_iter()
                    .map(|member| member.calculate_individual_scores(new_score))
                    .collect();
                GroupMember::Group(new_score, updated_members)
            }
            garbage => garbage,
        }
    }

    pub fn total_score(&self) -> u32 {
        match *self {
            GroupMember::Group(ref score, ref members) => {
                score +
                    members
                        .iter()
                        .map(|ref member| member.total_score())
                        .sum::<u32>()
            }
            GroupMember::Garbage(_) => 0,
        }
    }

    pub fn total_noncancelled_garbage(&self) -> usize {
        match *self {
            GroupMember::Group(_, ref members) => {
                members
                    .iter()
                    .map(|ref member| member.total_noncancelled_garbage())
                    .sum::<usize>()
            }
            GroupMember::Garbage(ref contents) => contents.len(),
        }
    }
}

pub struct Day09 {
    input: GroupMember,
}

impl<'a> Day<'a> for Day09 {
    const NUM: u32 = 9;
    type Output1 = u32;
    type Output2 = usize;

    fn from_str(input: &'a str) -> Self {
        Day09 { input: GroupMember::from_str(input) }
    }

    fn part_1(&self) -> Self::Output1 {
        self.input.total_score()
    }

    fn part_2(&self) -> Self::Output2 {
        self.input.total_noncancelled_garbage()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_tests() {
        assert_eq!(1, GroupMember::from_str("{}").total_score());
        assert_eq!(6, GroupMember::from_str("{{{}}}").total_score());
        assert_eq!(5, GroupMember::from_str("{{},{}}").total_score());
        assert_eq!(16, GroupMember::from_str("{{{},{},{{}}}}").total_score());
        assert_eq!(1, GroupMember::from_str("{<a>,<a>,<a>,<a>}").total_score());
        assert_eq!(
            9,
            GroupMember::from_str("{{<ab>},{<ab>},{<ab>},{<ab>}}").total_score()
        );
        assert_eq!(
            9,
            GroupMember::from_str("{{<!!>},{<!!>},{<!!>},{<!!>}}").total_score()
        );
        assert_eq!(
            3,
            GroupMember::from_str("{{<a!>},{<a!>},{<a!>},{<ab>}}").total_score()
        );
    }

    #[test]
    fn total_noncancelled_garbage() {
        assert_eq!(
            0,
            GroupMember::from_str("{<>}").total_noncancelled_garbage()
        );
        assert_eq!(
            17,
            GroupMember::from_str("{<random characters>}").total_noncancelled_garbage()
        );
        assert_eq!(
            3,
            GroupMember::from_str("{<<<<>}").total_noncancelled_garbage()
        );
        assert_eq!(
            2,
            GroupMember::from_str("{<{!>}>}").total_noncancelled_garbage()
        );
        assert_eq!(
            0,
            GroupMember::from_str("{<!!>}").total_noncancelled_garbage()
        );
        assert_eq!(
            0,
            GroupMember::from_str("{<!!!>>}").total_noncancelled_garbage()
        );
        assert_eq!(
            10,
            GroupMember::from_str("{<{o\"i!a,<{i<a>}").total_noncancelled_garbage()
        );
    }
}