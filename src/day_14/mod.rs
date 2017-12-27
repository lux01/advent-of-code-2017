mod disk_grid;

use super::Day;
use self::disk_grid::DiskGrid;

pub struct Day14 {
    grid: DiskGrid,
}

impl<'a> Day<'a> for Day14 {
    const NUM: u32 = 14;
    type Output1 = usize;
    type Output2 = usize;

    fn from_str(input: &str) -> Self {
        let mut grid = DiskGrid::from_str(input);
        grid.calculate_regions();
        Day14 { grid }
    }

    fn part_1(&self) -> Self::Output1 {
        self.grid.total_used()
    }

    fn part_2(&self) -> Self::Output2 {
        self.grid.total_regions()
    }
}
