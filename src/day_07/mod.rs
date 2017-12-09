mod parser;
mod program;

use super::Day;

pub struct Day07<'a> {
    program_tree: program::ProgramTree<'a>,
}

impl<'a> Day<'a> for Day07<'a> {
    const NUM: u32 = 7;
    type Output = &'a str;

    fn from_str(input: &'a str) -> Self {
        let program_tree = program::ProgramTree::from_str(input);
        Day07 { program_tree: program_tree.resolve_parents() }
    }

    fn part_1(&self) -> Self::Output {
        self.program_tree.bottom_programs[0].borrow().name
    }

    fn part_2(&self) -> Self::Output {
        ""
    }
}