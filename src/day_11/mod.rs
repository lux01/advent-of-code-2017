use super::Day;
use std::cmp;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HexCoord {
    axial_x: i32,
    axial_y: i32,
}

impl HexCoord {
    fn north_east(self) -> Self {
        HexCoord {
            axial_x: self.axial_x + 1,
            axial_y: self.axial_y - 1,
        }
    }

    fn north(self) -> Self {
        HexCoord {
            axial_x: self.axial_x,
            axial_y: self.axial_y - 1,
        }
    }

    fn north_west(self) -> Self {
        HexCoord {
            axial_x: self.axial_x - 1,
            axial_y: self.axial_y,
        }
    }

    fn south_west(self) -> Self {
        HexCoord {
            axial_x: self.axial_x - 1,
            axial_y: self.axial_y + 1,
        }
    }

    fn south(self) -> Self {
        HexCoord {
            axial_x: self.axial_x,
            axial_y: self.axial_y + 1,
        }
    }

    fn south_east(self) -> Self {
        HexCoord {
            axial_x: self.axial_x + 1,
            axial_y: self.axial_y,
        }
    }

    fn distance_from(&self, other: HexCoord) -> i32 {
        ((self.axial_x - other.axial_x).abs() + (self.axial_y - other.axial_y).abs() +
             (self.axial_x + self.axial_y - other.axial_x - other.axial_y)) / 2
    }

    fn steps_away(&self) -> i32 {
        self.distance_from(HexCoord::default())
    }
}

pub struct Day11<'a> {
    input: &'a str,
}

impl<'a> Day<'a> for Day11<'a> {
    const NUM: u32 = 11;
    type Output1 = i32;
    type Output2 = i32;

    fn from_str(input: &'a str) -> Self {
        Day11 { input }
    }

    fn part_1(&self) -> Self::Output1 {
        self.input
            .split(",")
            .fold(HexCoord::default(), |hex, direction| match direction {
                "ne" => hex.north_east(),
                "n" => hex.north(),
                "nw" => hex.north_west(),
                "sw" => hex.south_west(),
                "s" => hex.south(),
                "se" => hex.south_east(),
                x => unreachable!(x),
            })
            .steps_away()
    }

    fn part_2(&self) -> Self::Output2 {
        self.input
            .split(",")
            .fold((HexCoord::default(), 0), |(hex, max_dist), direction| {
                let next_hex = match direction {
                    "ne" => hex.north_east(),
                    "n" => hex.north(),
                    "nw" => hex.north_west(),
                    "sw" => hex.south_west(),
                    "s" => hex.south(),
                    "se" => hex.south_east(),
                    x => unreachable!(x),
                };
                let new_max_dist = cmp::max(max_dist, next_hex.steps_away());
                (next_hex, new_max_dist)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_distances() {
        assert_eq!(
            3,
            HexCoord::default()
                .north_east()
                .north_east()
                .north_east()
                .steps_away()
        );

        assert_eq!(
            0,
            HexCoord::default()
                .north_east()
                .north_east()
                .south_west()
                .south_west()
                .steps_away()
        );

        assert_eq!(
            2,
            HexCoord::default()
                .north_east()
                .north_east()
                .south()
                .south()
                .steps_away()
        );

        assert_eq!(
            3,
            HexCoord::default()
                .south_east()
                .south_west()
                .south_east()
                .south_west()
                .south_west()
                .steps_away()
        );
    }
}