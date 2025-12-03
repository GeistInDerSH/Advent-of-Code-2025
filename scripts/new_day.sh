#!/bin/bash

DAY=$1

mkdir -pv inputs/${DAY}
touch inputs/${DAY}/input.part1 inputs/${DAY}/input.part2 inputs/${DAY}/input.sample

echo "use crate::{read_to_iter, Input, Solution};

const DAY: u8 = ${DAY};

pub struct Day${DAY};

impl Solution<usize, usize> for Day${DAY} {
    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}

impl From<Input> for Day${DAY} {
    fn from(value: Input) -> Self {
        let _iter = read_to_iter(&value);
        Day${DAY} {}
    }
}

pub fn run() {
    let day = Day${DAY}::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day${DAY}::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part1());
        assert_eq!(0, 0);
    }

    #[test]
    fn part_2() {
        let day_sample = Day${DAY}::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part2());
        assert_eq!(0, 0);
    }
}
" > src/day_${DAY}.rs