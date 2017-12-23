mod link;

use super::Day;

use petgraph::graph::{DefaultIx, NodeIndex, UnGraph};
use petgraph::visit::Bfs;

pub struct ProgramVillage {
    graph: UnGraph<usize, usize>,
    villagers: Vec<NodeIndex<DefaultIx>>,
}

impl ProgramVillage {
    pub fn new(num_villagers: usize) -> ProgramVillage {
        let mut graph = UnGraph::default();
        let mut villagers = Vec::with_capacity(num_villagers);

        for i in 0..num_villagers {
            villagers.insert(i, graph.add_node(i));
        }

        ProgramVillage { graph, villagers }
    }

    pub fn populate_village(&mut self, input: &str) {
        let all_links = link::Link::from_str(input);

        for link in all_links.into_iter() {
            let from = self.villagers[link.from];
            let to = self.villagers[link.to];
            self.graph.add_edge(from, to, 1);
        }
    }

    pub fn calculate_groups(&self) -> Vec<usize> {
        let mut output = vec![0; self.villagers.len()];
        let mut next_usued_group = 1;

        // Calculate the first ungrouped villager
        while let Some((villager, _)) =
            output
                .iter()
                .enumerate()
                .filter(|&(_, &group)| group == 0)
                .next()
        {
            let mut bfs = Bfs::new(&self.graph, self.villagers[villager]);

            output[villager] = next_usued_group;

            while let Some(node) = bfs.next(&self.graph) {
                output[self.graph[node]] = next_usued_group;
            }

            next_usued_group += 1;
        }

        output
    }
}

pub struct Day12 {
    village: ProgramVillage,
}

impl<'a> Day<'a> for Day12 {
    const NUM: u32 = 12;
    type Output1 = usize;
    type Output2 = usize;

    fn from_str(input: &str) -> Self {
        let num_lines = input.lines().count();
        let mut village = ProgramVillage::new(num_lines);
        village.populate_village(input);
        Day12 { village }
    }

    fn part_1(&self) -> Self::Output1 {
        let groups = self.village.calculate_groups();

        groups.into_iter().filter(|&group| group == 1).count()
    }

    fn part_2(&self) -> Self::Output2 {
        self.village.calculate_groups().into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn part_1() {
        assert_eq!(6, Day12::from_str(&INPUT).part_1());
    }

    #[test]
    fn groups() {
        let groups = vec![1, 2, 1, 1, 1, 1, 1];
        assert_eq!(groups, Day12::from_str(&INPUT).village.calculate_groups());
    }

    #[test]
    fn part_2() {
        assert_eq!(2, Day12::from_str(&INPUT).part_2());
    }
}