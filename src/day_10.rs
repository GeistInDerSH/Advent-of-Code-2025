use crate::{read_to_iter, Input, Solution};
use itertools::Itertools;
use std::str::FromStr;

const DAY: u8 = 10;

type BitMask = u16;

#[derive(Clone, Debug)]
struct Machine {
    expected_state: BitMask,
    button_presses: Vec<BitMask>,
    joltage: Vec<u16>,
}

impl From<String> for Machine {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split(' ').collect();
        // [.##.] -> 0b0110 -> 6
        let expected_state = {
            let bytes = &parts[0].as_bytes()[1..parts[0].len() - 1];
            let mut value = 0;
            for (i, b) in bytes.iter().enumerate() {
                if *b == b'#' {
                    value |= 1 << i;
                }
            }
            value
        };
        // (3) (1,3) (2) (2,3) (0,2) (0,1)
        // Drop the '(' and ')' for each, then convert to a u16 with the bits set. e.g.
        // 1,3 -> 0b1010 -> 10
        // This way we can XOR with the expected state
        let button_presses = parts[1..parts.len() - 1]
            .iter()
            .map(|tuple| {
                let digits = tuple[1..tuple.len() - 1].split(',').flat_map(u16::from_str);
                let mut mask = 0;
                for digit in digits {
                    mask |= 1 << digit;
                }
                mask
            })
            .collect::<Vec<u16>>();
        // {3,5,4,7}
        let joltage = {
            let joltage_str = parts.last().unwrap();
            joltage_str[1..joltage_str.len() - 1]
                .split(',')
                .flat_map(u16::from_str)
                .collect::<Vec<u16>>()
        };
        Machine {
            expected_state,
            button_presses,
            joltage,
        }
    }
}

pub struct Day10(Vec<Machine>);

impl Solution<usize, usize> for Day10 {
    fn part1(&self) -> usize {
        self.0
            .iter()
            .map(|machine| {
                machine
                    .button_presses
                    .iter()
                    .powerset()
                    .map(|combos| {
                        let value = combos.iter().fold(0, |acc, combo| acc ^ **combo);
                        (value, combos.len())
                    })
                    .filter(|(value, _)| *value == machine.expected_state)
                    .min_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap_or((0, usize::MAX))
                    .1
            })
            .sum()
    }

    fn part2(&self) -> usize {
        0
    }
}

impl From<Input> for Day10 {
    fn from(value: Input) -> Self {
        let machines = read_to_iter(&value).unwrap().map(Machine::from).collect();
        Day10(machines)
    }
}

pub fn run() {
    let day = Day10::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day10::from(Input::Sample(DAY));
        assert_eq!(7, day_sample.part1());
        let day_sample = Day10::from(Input::Part1(DAY));
        assert_eq!(409, day_sample.part1());
    }

    #[test]
    fn part_2() {
        let day_sample = Day10::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part2());
    }
}
