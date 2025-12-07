use crate::points::point_2d::Point2D;
use crate::{read_to_iter, Input, Solution};
use std::collections::{BinaryHeap, HashMap, HashSet};

const DAY: u8 = 7;
const WEST: TachyonSplitters = TachyonSplitters { row: 0, col: -1 };
const EAST: TachyonSplitters = TachyonSplitters { row: 0, col: 1 };

type TachyonSplitters = Point2D<i64>;

pub struct Day7 {
    start: TachyonSplitters,
    manifolds: HashSet<TachyonSplitters>,
    width: i64,
    height: i64,
}

impl Solution<usize, usize> for Day7 {
    fn part1(&self) -> usize {
        let mut heap = BinaryHeap::new();
        heap.push(self.start);
        let mut visited = HashSet::new();
        while let Some(TachyonSplitters { row, col }) = heap.pop() {
            let after = (row..self.height)
                .map(|row| TachyonSplitters { row, col })
                .find(|&next| self.manifolds.contains(&next));

            if let Some(next) = after
                && visited.insert(next)
            {
                heap.push(next + EAST);
                heap.push(next + WEST);
            }
        }

        visited.len()
    }

    fn part2(&self) -> usize {
        let mut beams = HashMap::new();
        beams.insert(self.start.col, 1);
        for row in 0..self.height {
            for col in 0..self.width {
                let point = TachyonSplitters { row, col };
                if !self.manifolds.contains(&point) {
                    continue;
                }

                if let Some(count) = beams.remove(&col) {
                    *beams.entry(col - 1).or_insert(0) += count;
                    *beams.entry(col + 1).or_insert(0) += count;
                }
            }
        }

        beams.into_values().sum()
    }
}

impl From<Input> for Day7 {
    fn from(value: Input) -> Self {
        let iter = read_to_iter(&value).unwrap().collect::<Vec<_>>();

        let width = i64::try_from(iter[0].len()).unwrap();
        let height = i64::try_from(iter.len()).unwrap();
        let mut start = Point2D::default();
        let mut tachyon_manifold = HashSet::new();
        for (row, line) in iter.iter().enumerate() {
            for (col, ch) in line.as_bytes().iter().enumerate() {
                match *ch {
                    b'S' => start = TachyonSplitters::new(row, col),
                    b'^' => {
                        tachyon_manifold.insert(TachyonSplitters::new(row, col));
                    }
                    _ => {}
                }
            }
        }

        Day7 {
            start,
            height,
            width,
            manifolds: tachyon_manifold,
        }
    }
}

pub fn run() {
    let day = Day7::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day7::from(Input::Sample(DAY));
        assert_eq!(21, day_sample.part1());
        assert_eq!(1640, 1640);
    }

    #[test]
    fn part_2() {
        let day_sample = Day7::from(Input::Sample(DAY));
        assert_eq!(40, day_sample.part2());
        assert_eq!(40_999_072_541_589, 40_999_072_541_589_usize);
    }
}
