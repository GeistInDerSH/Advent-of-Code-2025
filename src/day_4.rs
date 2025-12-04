use crate::{read_to_iter, Input, Solution};
use std::collections::HashSet;

const DAY: u8 = 4;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point2D {
    row: i16,
    col: i16,
}

impl Point2D {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: i16::try_from(row).unwrap(),
            col: i16::try_from(col).unwrap(),
        }
    }

    fn neighbors(self) -> [Point2D; 9] {
        let mut neighbors = [Point2D { row: 0, col: 0 }; 9];

        let mut index = 0;
        for row_inc in -1..=1 {
            let row = self.row + row_inc;
            neighbors[index] = Point2D {
                row,
                col: self.col - 1,
            };
            neighbors[index + 1] = Point2D { row, col: self.col };
            neighbors[index + 2] = Point2D {
                row,
                col: self.col + 1,
            };
            index += 3;
        }

        neighbors
    }
}

#[derive(Clone)]
pub struct Day4(HashSet<Point2D>);

impl Day4 {
    fn reachable_paper_rolls(&self) -> HashSet<Point2D> {
        self.0
            .iter()
            .filter(|&point| {
                let free_spots = point
                    .neighbors()
                    .iter()
                    .filter(|neighbor| self.0.contains(neighbor))
                    .count();
                free_spots <= 4
            })
            .copied()
            .collect()
    }
}

impl Solution<usize, usize> for Day4 {
    fn part1(&self) -> usize {
        self.reachable_paper_rolls().len()
    }

    fn part2(&self) -> usize {
        let mut points = self.clone();

        let mut remove_count = 0;
        let mut reachable = points.reachable_paper_rolls();
        while !reachable.is_empty() {
            for point in &reachable {
                points.0.remove(point);
            }
            remove_count += reachable.len();
            reachable = points.reachable_paper_rolls();
        }

        remove_count
    }
}

impl From<Input> for Day4 {
    fn from(value: Input) -> Self {
        let mut points = HashSet::new();

        for (row, line) in read_to_iter(&value).unwrap().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != '@' {
                    continue;
                }
                points.insert(Point2D::new(row, col));
            }
        }

        Day4(points)
    }
}

pub fn run() {
    let day = Day4::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day4::from(Input::Sample(DAY));
        assert_eq!(13, day_sample.part1());
        assert_eq!(1367, 1367);
    }

    #[test]
    fn part_2() {
        let day_sample = Day4::from(Input::Sample(DAY));
        assert_eq!(43, day_sample.part2());
        assert_eq!(9144, 9144);
    }
}
