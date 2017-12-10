use super::parser;


use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

// Rc = Reference counted pointer
// RefCell = A mutable memory location on the heap with borrow rules checked dynamically.
//           Borrows are done by reference rather than by value.
type ProgramRef<'a> = Rc<RefCell<Program<'a>>>;

#[derive(Eq)]
pub struct Program<'a> {
    pub name: &'a str,
    pub weight: u64,
    pub children: Vec<ProgramRef<'a>>,
    pub parent: Option<ProgramRef<'a>>,
}


impl<'a> Program<'a> {
    pub fn new(name: &'a str, weight: u64) -> Program<'a> {
        Program {
            name: name,
            weight: weight,
            children: vec![],
            parent: None,
        }
    }

    pub fn total_weight(&self) -> u64 {
        self.weight +
            self.children
                .iter()
                .map(|child| child.borrow().total_weight())
                .sum::<u64>()
    }

    pub fn add_child(&mut self, child: ProgramRef<'a>) {
        self.children.push(child);
        self.children.sort();
    }

    pub fn childrens_weights(&self) -> Vec<u64> {
        let mut all_childrens_weights = self.children
            .iter()
            .map(|child| child.borrow().total_weight())
            .collect::<Vec<_>>();
        all_childrens_weights.dedup();
        all_childrens_weights
    }

    pub fn has_incorrect_weight(&self) -> bool {
        match self.parent {
            None => false,
            Some(ref parent) => {
                let all_siblings_weight = parent.borrow().childrens_weights();
                if all_siblings_weight.len() == 1 {
                    return false;
                } else {
                    let my_weight = self.total_weight();

                    let instances_of_my_weight = all_siblings_weight
                        .iter()
                        .filter(|&&weight| weight == my_weight)
                        .count();

                    instances_of_my_weight == 1
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct ProgramTree<'a> {
    pub programs: HashMap<&'a str, ProgramRef<'a>>,
    pub bottom_programs: Vec<ProgramRef<'a>>,
}

impl<'a> ProgramTree<'a> {
    pub fn from_str(input: &'a str) -> ProgramTree<'a> {
        let (unused, parse_results) = parser::many_programs(input).unwrap();
        assert_eq!("", unused);

        let mut parent_children_names = HashMap::new();
        let mut programs = HashMap::new();

        for (program, children_names) in parse_results.into_iter() {
            parent_children_names.insert(program.name, children_names);
            programs.insert(program.name, Rc::new(RefCell::new(program)));
        }

        for (parent_name, children_names) in parent_children_names.into_iter() {
            for child_name in children_names.into_iter() {
                let ref child = programs[&child_name];
                let ref parent = programs[&parent_name];

                child.borrow_mut().parent = Some(parent.clone());
                parent.borrow_mut().children.push(child.clone());
            }
        }

        let bottom_programs = programs
            .iter()
            .filter(|&(_, program)| program.borrow().parent.is_none())
            .map(|(_, program)| program.clone())
            .collect();

        let output = ProgramTree {
            programs,
            bottom_programs,
        };

        output
    }
}

// We need to manually implement Debug, PartialOrd, Ord, and PartialEq for Program due
// to the circular references between parents and children. If we try to derive these
// traits automatically they will get stuck in an endless loop and smash the stack.

impl<'a> fmt::Debug for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({})", self.name, self.weight)?;

        if self.children.len() > 0 {
            write!(f, " -> {}", self.children[0].borrow().name)?;
        }

        for child in self.children.iter().skip(1) {
            write!(f, ", {}", child.borrow().name)?;
        }
        Ok(())
    }
}

impl<'a> PartialOrd for Program<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.name.cmp(&other.name).then(
            self.weight.cmp(&other.weight),
        ))
    }
}

impl<'a> Ord for Program<'a> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name).then(
            self.weight.cmp(&other.weight),
        )
    }
}

impl<'a> PartialEq for Program<'a> {
    fn eq(&self, other: &Self) -> bool {
        let name = self.name == other.name;
        let weight = self.weight == other.weight;

        name && weight
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
        let c = Rc::new(RefCell::new(Program::new("c", 3)));

        a.borrow_mut().parent = Some(c.clone());
        b.borrow_mut().parent = Some(c.clone());
        c.borrow_mut().add_child(a.clone());
        c.borrow_mut().add_child(b.clone());

        let mut expected_programs = HashMap::new();
        expected_programs.insert("a", a.clone());
        expected_programs.insert("b", b.clone());
        expected_programs.insert("c", c.clone());

        assert_eq!(expected_programs, tree.programs);
        assert!(!tree.bottom_programs.is_empty());
    }

}