use crate::points::point_2d::Point2D;
use crate::{read_to_iter, Input, Solution};
use std::collections::BinaryHeap;

const DAY: u8 = 9;

type Light = Point2D<i32>;
type Distance = usize;
type RowMin = i32;
type RowMax = i32;
type ColMin = i32;
type ColMax = i32;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct LightPair((Light, Light));

impl LightPair {
    fn area(&self) -> usize {
        let width = (self.0.1.col - self.0.0.col).unsigned_abs() as usize + 1;
        let height = (self.0.1.row - self.0.0.row).unsigned_abs() as usize + 1;
        width * height
    }

    fn corners(&self) -> ((RowMin, RowMax), (ColMin, ColMax)) {
        let row = (
            self.0.0.row.min(self.0.1.row),
            self.0.0.row.max(self.0.1.row),
        );
        let col = (
            self.0.0.col.min(self.0.1.col),
            self.0.0.col.max(self.0.1.col),
        );
        (row, col)
    }
}

pub struct Day9(Vec<Light>);

impl Day9 {
    fn pairs(&self) -> BinaryHeap<(Distance, LightPair)> {
        (0..self.0.len())
            .flat_map(|a| (a + 1..self.0.len()).map(move |b| (a, b)))
            .map(|(a, b)| {
                let p1 = self.0[a];
                let p2 = self.0[b];
                let pair = LightPair((p1, p2));
                (pair.area(), pair)
            })
            .collect::<BinaryHeap<_>>()
    }
}

impl Solution<usize, usize> for Day9 {
    fn part1(&self) -> usize {
        self.pairs()
            .iter()
            .map(|(_, pair)| pair.area())
            .max()
            .unwrap_or(0)
    }

    fn part2(&self) -> usize {
        self.pairs()
            .iter()
            .map(|(_, pair)| pair)
            .filter_map(|pair| {
                let ((row_min, row_max), (col_min, col_max)) = pair.corners();

                for (i, start) in self.0.iter().enumerate() {
                    let end = self.0[(i + 1) % self.0.len()];

                    if start.row == end.row {
                        let (col_start, col_end) = (start.col.min(end.col), start.col.max(end.col));
                        if row_min < start.row
                            && row_max > start.row
                            && !(col_min >= col_end || col_max <= col_start)
                        {
                            return None;
                        }
                    } else if start.col == end.col {
                        let (row_start, row_end) = (start.row.min(end.row), start.row.max(end.row));
                        if col_min < start.col
                            && col_max > start.col
                            && !(row_min >= row_end || row_max <= row_start)
                        {
                            return None;
                        }
                    } else {
                        // This should never happen!
                        return None;
                    }
                }

                Some(pair.area())
            })
            .max()
            .unwrap_or(0)
    }
}

impl From<Input> for Day9 {
    fn from(value: Input) -> Self {
        let lights = read_to_iter(&value)
            .unwrap()
            .flat_map(Light::try_from)
            .collect::<Vec<Light>>();
        Day9(lights)
    }
}

pub fn run() {
    let day = Day9::from(Input::Part1(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day9::from(Input::Sample(DAY));
        assert_eq!(50, day_sample.part1());
        assert_eq!(4_738_108_384, 4_738_108_384_usize);
    }

    #[test]
    fn part_2() {
        let day_sample = Day9::from(Input::Sample(DAY));
        assert_eq!(24, day_sample.part2());
        assert_eq!(1_513_792_010, 1_513_792_010);
    }
}
