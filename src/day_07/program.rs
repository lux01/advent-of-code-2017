use super::parser;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct Program<'a> {
    pub name: &'a str,
    pub weight: u32,
    pub children: Vec<&'a str>,
    pub maybe_parent: Option<&'a str>,
}

impl<'a> Program<'a> {
    pub fn new(name: &'a str, weight: u32) -> Program<'a> {
        Program {
            name: name,
            weight: weight,
            children: vec![],
            maybe_parent: None,
        }
    }

    pub fn children(self, children: Vec<&'a str>) -> Program<'a> {
        Program {
            children: children,
            ..self
        }
    }

    pub fn parent(self, parent: &'a str) -> Program<'a> {
        Program {
            maybe_parent: Some(parent),
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProgramTree<'a> {
    pub programs: HashMap<&'a str, Rc<RefCell<Program<'a>>>>,
    pub bottom_programs: Vec<Rc<RefCell<Program<'a>>>>,
}

impl<'a> ProgramTree<'a> {
    pub fn from_str(input: &str) -> ProgramTree {
        let (unused, results) = parser::many_programs(input).unwrap();
        assert_eq!("", unused);

        let mut programs = HashMap::new();
        for program in results.into_iter() {
            programs.insert(program.name, Rc::new(RefCell::new(program)));
        }

        ProgramTree {
            programs,
            bottom_programs: vec![],
        }
    }

    pub fn resolve_parents(self) -> ProgramTree<'a> {
        for (name, program) in self.programs.iter() {
            for child_name in program.borrow().children.iter() {
                self.programs[child_name].borrow_mut().maybe_parent = Some(name);
            }
        }

        let bottom_programs = self.programs
            .iter()
            .filter(|&(_, program)| program.borrow().maybe_parent.is_none())
            .map(|(_, program)| program.clone())
            .collect();

        ProgramTree {
            bottom_programs,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_tree_from_str() {
        let input = "a (1)
        b (2)
        c (3) -> a, b";

        let tree = ProgramTree::from_str(&input);

        let a = Rc::new(RefCell::new(Program::new("a", 1)));
        let b = Rc::new(RefCell::new(Program::new("b", 2)));
        let c = Rc::new(RefCell::new(Program::new("c", 3).children(vec!["a", "b"])));

        let mut expected_programs = HashMap::new();
        expected_programs.insert("a", a);
        expected_programs.insert("b", b);
        expected_programs.insert("c", c);

        assert_eq!(expected_programs, tree.programs);
        assert!(tree.bottom_programs.is_empty());
    }

    #[test]
    fn program_tree_resolve_parents() {
        let input = "a (1)
        b (2)
        c (3) -> a, b";

        let tree = ProgramTree::from_str(&input).resolve_parents();


        let a = Rc::new(RefCell::new(Program::new("a", 1).parent("c")));
        let b = Rc::new(RefCell::new(Program::new("b", 2).parent("c")));
        let c = Rc::new(RefCell::new(Program::new("c", 3).children(vec!["a", "b"])));

        let mut expected_programs = HashMap::new();
        expected_programs.insert("a", a);
        expected_programs.insert("b", b);
        expected_programs.insert("c", c.clone());

        let bottom_programs = vec![c.clone()];

        assert_eq!(
            expected_programs,
            tree.programs,
            "Output tree does not have the correct program list"
        );
        assert_eq!(
            bottom_programs,
            tree.bottom_programs,
            "Output tree does not have the correct bottom programs list"
        );
    }

}