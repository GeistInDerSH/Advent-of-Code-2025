use crate::{Input, Solution};
use std::ops::{Add, Mul};

const DAY: u8 = 6;

type ReducedRow = usize;
type ReducedColumn = usize;
type ReduceFn = fn(usize, usize) -> usize;

struct Column((ReducedRow, ReducedColumn));

impl From<&[&str]> for Column {
    fn from(lines: &[&str]) -> Self {
        let reduce_fn: ReduceFn = {
            let reduce_symbol = lines.last().unwrap().trim();
            match reduce_symbol {
                "*" => usize::mul,
                "+" => usize::add,
                _ => panic!("Unknown Symbol: {reduce_symbol}"),
            }
        };

        let value_lines = &lines[0..lines.len() - 1];
        let reduced_row: ReducedRow = value_lines
            .iter()
            .map(|l| l.trim().parse().unwrap())
            .reduce(reduce_fn)
            .unwrap_or(0);
        let reduced_col: ReducedColumn = parse_column_wise_and_reduce(value_lines, reduce_fn);

        Self((reduced_row, reduced_col))
    }
}

pub struct Day6(Vec<Column>);

impl Solution<usize, usize> for Day6 {
    fn part1(&self) -> usize {
        self.0.iter().map(|c| c.0.0).sum()
    }

    fn part2(&self) -> usize {
        self.0.iter().map(|c| c.0.1).sum()
    }
}

impl From<Input> for Day6 {
    fn from(value: Input) -> Self {
        let columns = {
            let string = std::fs::read_to_string(value.to_string()).unwrap();
            let lines = string.lines().collect::<Vec<&str>>();

            let mut columns = vec![];
            let mut starting_column = 0;
            // The last line is the operators, and they are formatted so they always start the
            // column. We can use that to avoid needing to find where the break between columns is.
            for (i, ch) in lines[lines.len() - 1].chars().enumerate() {
                if ch == ' ' || i == 0 {
                    continue;
                }

                // Get all values in the current problem between the starting and ending column.
                let values = lines
                    .iter()
                    .map(|line| &line[starting_column..i - 1])
                    .collect::<Vec<&str>>();
                columns.push(Column::from(values.as_slice()));
                starting_column = i;
            }

            // Account for the last column which won't have an ending space.
            {
                let values = lines
                    .iter()
                    .map(|line| &line[starting_column..])
                    .collect::<Vec<&str>>();
                columns.push(Column::from(values.as_slice()));
            }
            columns
        };

        Day6(columns)
    }
}

/// Read the values in the columns from right-to-left and return them as a vector.
///
/// For example:
/// ```markdown
/// 123
///  45
///   6
/// ```
/// would return:
/// ```markdown
/// [356, 24, 1]
/// ```
///
fn parse_column_wise_and_reduce(lines: &[&str], reduce_fn: ReduceFn) -> ReducedColumn {
    let longest = lines.iter().map(|s| s.len()).max().unwrap();

    // Parse each of the rows top to bottom, right to left. These get stored into
    // the vector in reverse order, so we need to handle that before returning.
    // Example:
    // 123
    //  45
    //   6
    // Gets pushed as: [1, 24, 356]
    let mut columnar_values = vec![0; longest];
    for row in lines {
        for (i, c) in row.as_bytes().iter().enumerate().rev() {
            if !c.is_ascii_digit() {
                continue;
            }

            columnar_values[i] *= 10;
            columnar_values[i] += (c - b'0') as usize;
        }
    }

    // Ensure the right-to-left order is maintained for correctness when reducing
    columnar_values
        .iter()
        .rev()
        .copied()
        .reduce(reduce_fn)
        .unwrap_or(0)
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
        assert_eq!(11_950_004_808_442, 11_950_004_808_442_usize);
    }
}
