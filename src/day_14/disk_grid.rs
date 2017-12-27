use std::collections::HashMap;
use std::fmt::Write;

use super::super::util::knot_hash::KnotHash;

#[derive(Debug, PartialEq, Eq)]
pub struct DiskGrid {
    grid: HashMap<(u8, u8), Status>,
    regions: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Free,
    Used,
    Region(usize),
}

impl Status {
    pub fn is_region(&self) -> bool {
        match *self {
            Status::Region(_) => true,
            _ => false,
        }
    }

    pub fn is_free(&self) -> bool {
        match *self {
            Status::Free => true,
            _ => false,
        }
    }

    pub fn is_used(&self) -> bool {
        match *self {
            Status::Used => true,
            _ => false,
        }
    }
}

impl From<bool> for Status {
    fn from(b: bool) -> Status {
        if b {
            Status::Used
        } else {
            Status::Free
        }
    }
}

impl DiskGrid {
    pub fn from_str(input: &str) -> DiskGrid {
        let mut grid: HashMap<(u8, u8), Status> = HashMap::with_capacity(128 * 128);
        for j in 0..128 {
            let input_str = format!("{}-{}", input, j);
            let bytes = KnotHash::hash_str(&input_str);

            for i_major in 0..16 {
                let byte = bytes[i_major];
                let i_0 = byte & 0b00000001 == 0b00000001;
                let i_1 = byte & 0b00000010 == 0b00000010;
                let i_2 = byte & 0b00000100 == 0b00000100;
                let i_3 = byte & 0b00001000 == 0b00001000;
                let i_4 = byte & 0b00010000 == 0b00010000;
                let i_5 = byte & 0b00100000 == 0b00100000;
                let i_6 = byte & 0b01000000 == 0b01000000;
                let i_7 = byte & 0b10000000 == 0b10000000;

                grid.insert((i_major * 8 + 0, j), i_7.into());
                grid.insert((i_major * 8 + 1, j), i_6.into());
                grid.insert((i_major * 8 + 2, j), i_5.into());
                grid.insert((i_major * 8 + 3, j), i_4.into());
                grid.insert((i_major * 8 + 4, j), i_3.into());
                grid.insert((i_major * 8 + 5, j), i_2.into());
                grid.insert((i_major * 8 + 6, j), i_1.into());
                grid.insert((i_major * 8 + 7, j), i_0.into());
            }
        }

        DiskGrid { grid, regions: 0 }
    }

    pub fn total_used(&self) -> usize {
        self.grid
            .iter()
            .filter(|&(_, status)| status.is_used() || status.is_region())
            .count()
    }

    pub fn to_str(&self, width: u8, height: u8) -> String {
        let mut string_buffer = String::with_capacity(width as usize * height as usize);

        for j in 0..height {
            for i in 0..width {
                write!(
                    &mut string_buffer,
                    "{}",
                    if self.grid[&(i, j)].is_free() {
                        '.'
                    } else {
                        '#'
                    }
                ).unwrap();
            }
            write!(&mut string_buffer, "\n").unwrap();
        }
        string_buffer
    }

    pub fn total_regions(&self) -> usize {
        self.regions
    }

    pub fn calculate_regions(&mut self) {
        let mut region = 0;

        for j in 0..128 {
            for i in 0..128 {
                let pos = (i, j);
                if self.grid[&pos].is_used() {
                    self.grid.insert(pos, Status::Region(region));
                    self.update_neighbours(pos, region);

                    region += 1;
                }
            }
        }

        self.regions = region;
    }

    fn update_neighbours(&mut self, pos: (u8, u8), region: usize) {
        let neighbours = self.used_neighbours(pos);

        for neighbour_pos in neighbours.iter() {
            self.grid.insert(*neighbour_pos, Status::Region(region));
        }
        for neighbour_pos in neighbours.into_iter() {
            self.update_neighbours(neighbour_pos, region);
        }
    }

    fn used_neighbours(&self, (x, y): (u8, u8)) -> Vec<(u8, u8)> {
        let immediate_neighbours = vec![
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        immediate_neighbours
            .into_iter()
            .filter(|pos| self.grid.get(pos).unwrap_or(&Status::Free).is_used())
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_used_count() {
        assert_eq!(8108, DiskGrid::from_str("flqrgnkx").total_used());
    }

    #[test]
    fn total_region_count() {
        let mut grid = DiskGrid::from_str("flqrgnkx");
        grid.calculate_regions();
        assert_eq!(1242, grid.total_regions());
    }

    #[test]
    fn immediate_neighbours() {
        let grid = DiskGrid::from_str("flqrgnkx");

        assert_eq!(vec![(1, 0)], grid.used_neighbours((0, 0)));
    }

    #[test]
    fn to_string() {
        let expected_output = "##.#.#..
.#.#.#.#
....#.#.
#.#.##.#
.##.#...
##..#..#
.#...#..
##.#.##.
";

        let input_str = "flqrgnkx";
        let width = 8;
        let height = 8;

        let grid = DiskGrid::from_str(input_str);

        assert_eq!(expected_output, grid.to_str(width, height));
    }
}
