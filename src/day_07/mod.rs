mod parser;
mod program;

use super::Day;

pub struct Day07<'a> {
    program_tree: program::ProgramTree<'a>,
}

impl<'a> Day<'a> for Day07<'a> {
    const NUM: u32 = 7;
    type Output1 = &'a str;
    type Output2 = i64;

    fn from_str(input: &'a str) -> Self {
        let program_tree = program::ProgramTree::from_str(input);
        Day07 { program_tree }
    }

    fn part_1(&self) -> Self::Output1 {
        self.program_tree.bottom_programs[0].borrow().name
    }

    fn part_2(&self) -> i64 {
        for (_, prog) in self.program_tree.programs.iter() {
            let program = prog.borrow();
            if program.has_incorrect_weight() {
                match program.parent {
                    Some(ref parent) => {
                        let siblings_weights = parent.borrow().childrens_weights();
                        let my_total_weight = program.total_weight() as i64;
                        let correct_total_weight = siblings_weights
                            .into_iter()
                            .filter(|weight| *weight as i64 != my_total_weight)
                            .next()
                            .unwrap() as i64;

                        return correct_total_weight - my_total_weight + program.weight as i64;
                    }
                    None => unreachable!(),
                }
            }
        }

        unreachable!()
    }
}