use crate::points::point_2d::Point2D;
use crate::{read_to_iter, Input, Solution};
use std::collections::BinaryHeap;

const DAY: u8 = 9;

type Light = Point2D<i32>;
type Distance = usize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct LightPair((Light, Light));

impl LightPair {
    fn area(&self) -> usize {
        let width = (self.0.1.col - self.0.0.col).unsigned_abs() as usize + 1;
        let height = (self.0.1.row - self.0.0.row).unsigned_abs() as usize + 1;
        width * height
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
                (p1.distance(p2), LightPair((p1, p2)))
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
        0
    }
}

impl From<Input> for Day9 {
    fn from(value: Input) -> Self {
        let lights = read_to_iter(&value)
            .unwrap()
            .map(|l| {
                let (lhs, rhs) = l.split_once(',').unwrap();
                Light::new(lhs.parse().unwrap(), rhs.parse().unwrap())
            })
            .collect();
        Day9(lights)
    }
}

pub fn run() {
    let day = Day9::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day9::from(Input::Sample(DAY));
        assert_eq!(50, day_sample.part1());
        let day_sample = Day9::from(Input::Part1(DAY));
        assert_eq!(4_738_108_384, day_sample.part1());
    }

    #[test]
    fn part_2() {
        let day_sample = Day9::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part2());
        assert_eq!(0, 0);
    }
}
