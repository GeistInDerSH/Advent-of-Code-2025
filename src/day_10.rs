use crate::{read_to_iter, Input, Solution};
use itertools::Itertools;
use std::str::FromStr;

const DAY: u8 = 10;

type BitMask = u16;

#[derive(Clone, Debug)]
struct Machine {
    expected_state: BitMask,
    button_presses: Vec<Vec<u16>>,
    joltage: Vec<u16>,
}

impl Machine {
    fn buttons_to_mask(&self) -> impl Iterator<Item = BitMask> {
        self.button_presses
            .iter()
            .map(|press| press.iter().fold(0, |acc, &x| acc | (1 << x)))
    }

    fn find_minimum_button_presses(&self) -> usize {
        let optimizer = z3::Optimize::new();
        let result = z3::ast::Int::fresh_const("result");

        // Create inputs for each of the button presses
        let presses = (0..self.button_presses.len())
            .map(|i| z3::ast::Int::fresh_const(&format!("press_{i}")))
            .collect::<Vec<_>>();

        // Ensure we can't press a button a negative number of times
        for press in &presses {
            optimizer.assert(&press.ge(0));
        }

        for (i, &joltage) in self.joltage.iter().enumerate() {
            let mut terms = Vec::new();

            for (j, press) in self.button_presses.iter().enumerate() {
                if press.contains(&u16::try_from(i).unwrap()) {
                    terms.push(presses[j].clone());
                }
            }

            let sum = z3::ast::Int::add(&terms);
            let z3_jolatage = z3::ast::Int::from_u64(u64::from(joltage));
            optimizer.assert(&sum.eq(z3_jolatage));
        }

        // Get the minimal result, that is the sum of button presses
        optimizer.assert(&result.eq(z3::ast::Int::add(&presses)));
        optimizer.minimize(&result);

        // Check the result of the z3 formula, and return 0 if there are any failures
        match optimizer.check(&[]) {
            z3::SatResult::Sat => match optimizer.get_model() {
                None => 0,
                Some(model) => {
                    let answer = model
                        .eval(&result, true)
                        .clone()
                        .and_then(|r| z3::ast::Int::as_u64(&r))
                        .unwrap_or(0);
                    usize::try_from(answer).unwrap_or(0)
                }
            },
            z3::SatResult::Unsat | z3::SatResult::Unknown => 0,
        }
    }
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
                tuple[1..tuple.len() - 1]
                    .split(',')
                    .flat_map(u16::from_str)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
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
                    .buttons_to_mask()
                    .powerset()
                    .map(|combos| {
                        let value = combos.iter().fold(0, |acc, combo| acc ^ combo);
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
        self.0
            .iter()
            .map(Machine::find_minimum_button_presses)
            .sum()
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
        assert_eq!(409, 409);
    }

    #[test]
    fn part_2() {
        let day_sample = Day10::from(Input::Sample(DAY));
        assert_eq!(33, day_sample.part2());
        assert_eq!(15489, 15489);
    }
}
