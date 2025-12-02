use crate::{Input, Solution};
use std::collections::HashSet;

const DAY: u8 = 2;

type Ranges = Vec<std::ops::RangeInclusive<usize>>;

pub struct Day2(Ranges);

impl Solution<usize, usize> for Day2 {
    fn part1(&self) -> usize {
        self.0
            .iter()
            .map(|range| {
                range
                    .clone()
                    .map(|i| (i, i.to_string()))
                    .filter(|(_, string)| has_repetition_twice(string))
                    .map(|(value, _)| value)
                    .sum::<usize>()
            })
            .sum()
    }

    fn part2(&self) -> usize {
        self.0
            .iter()
            .map(|range| {
                range
                    .clone()
                    .map(|i| (i, i.to_string()))
                    .filter(|(_, string)| {
                        let has_repetition = has_repetition_at_least_twice(string);
                        has_repetition || has_repetition_twice(string)
                    })
                    .map(|(value, _)| value)
                    .sum::<usize>()
            })
            .sum()
    }
}

impl From<Input> for Day2 {
    fn from(value: Input) -> Self {
        let contents = std::fs::read_to_string(value.to_string()).unwrap();
        let ranges = contents
            .split(',')
            .map(|raw| {
                let (lhs, rhs) = raw.split_once('-').unwrap();
                lhs.parse::<usize>().unwrap()..=rhs.parse::<usize>().unwrap()
            })
            .collect::<Vec<_>>();
        Day2(ranges)
    }
}

fn has_repetition_twice(string: &str) -> bool {
    let is_even_length = string.len().is_multiple_of(2);
    let (lhs, rhs) = string.split_at(string.len() / 2);
    is_even_length && lhs == rhs
}

fn has_repetition_at_least_twice(string: &str) -> bool {
    let bytes = string.as_bytes();
    // shortcut for all being the same char
    if bytes.iter().all(|&b| b == bytes[0]) {
        return true;
    }

    for window_size in 2..=string.len() / 2 {
        // Only consider windows that won't leave any remaining characters
        if !string.len().is_multiple_of(window_size) {
            continue;
        }

        let set = bytes
            .windows(window_size)
            .step_by(window_size)
            .collect::<HashSet<_>>();
        if set.len() == 1 {
            return true;
        }
    }
    false
}

pub fn run() {
    let day = Day2::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day2::from(Input::Sample(DAY));
        assert_eq!(1_227_775_554, day_sample.part1());
        let day_sample = Day2::from(Input::Part1(DAY));
        assert_eq!(37_314_786_486_usize, day_sample.part1());
    }

    #[test]
    fn part_2() {
        let day_sample = Day2::from(Input::Sample(DAY));
        assert_eq!(4_174_379_265, day_sample.part2());
        let day_sample = Day2::from(Input::Part1(DAY));
        assert_eq!(47_477_054_026, day_sample.part2());
    }
}
