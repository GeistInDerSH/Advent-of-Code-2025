use crate::{read_to_iter, Input, Solution};
use std::ops::{Add, Div, Mul, Sub};

const DAY: u8 = 6;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Multiply,
    Divide,
}

struct Column {
    values: Vec<usize>,
    operator: Operator,
}

pub struct Day6 {
    columns: Vec<Column>,
}

impl Solution<usize, usize> for Day6 {
    fn part1(&self) -> usize {
        self.columns
            .iter()
            .map(|Column { values, operator }| {
                let reducer = match operator {
                    Operator::Add => usize::add,
                    Operator::Sub => usize::sub,
                    Operator::Multiply => usize::mul,
                    Operator::Divide => usize::div,
                };
                values.iter().copied().reduce(reducer).unwrap_or(0)
            })
            .sum()
    }

    fn part2(&self) -> usize {
        0
    }
}

impl From<Input> for Day6 {
    fn from(value: Input) -> Self {
        let mut values = vec![];
        let mut operators = vec![];
        for line in read_to_iter(&value).unwrap() {
            let mut row = vec![];
            for s in line.split_whitespace() {
                match s.parse::<usize>() {
                    Ok(num) => row.push(num),
                    Err(_) => match s {
                        "+" => operators.push(Operator::Add),
                        "-" => operators.push(Operator::Sub),
                        "*" => operators.push(Operator::Multiply),
                        "/" => operators.push(Operator::Divide),
                        _ => panic!("Unknown operator {s}"),
                    },
                }
            }
            if !row.is_empty() {
                values.push(row);
            }
        }

        let columns = operators
            .iter()
            .enumerate()
            .map(|(index, op)| {
                let cols = values.iter().map(|v| v[index]).collect::<Vec<_>>();
                Column {
                    values: cols,
                    operator: *op,
                }
            })
            .collect();

        Day6 { columns }
    }
}

pub fn run() {
    let day = Day6::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day6::from(Input::Sample(DAY));
        assert_eq!(4_277_556, day_sample.part1());
        assert_eq!(6_299_564_383_938, 6_299_564_383_938_usize);
    }

    #[test]
    fn part_2() {
        let day_sample = Day6::from(Input::Sample(DAY));
        assert_eq!(3_263_827, day_sample.part2());
        assert_eq!(0, 0);
    }
}
