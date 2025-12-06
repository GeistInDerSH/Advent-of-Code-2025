use crate::{Input, Solution};
use std::ops::{Add, Div, Mul, Sub};

const DAY: u8 = 6;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Multiply,
    Divide,
}

impl Operator {
    fn to_reduce_fn(self) -> fn(usize, usize) -> usize {
        match self {
            Operator::Add => usize::add,
            Operator::Sub => usize::sub,
            Operator::Multiply => usize::mul,
            Operator::Divide => usize::div,
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Operator::Add),
            b'-' => Ok(Operator::Sub),
            b'*' => Ok(Operator::Multiply),
            b'/' => Ok(Operator::Divide),
            _ => Err(format!("Unknown operator: {value}")),
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(format!("Unknown operator: {value}")),
        }
    }
}

/// Columns contains the raw contents of a column as Strings, and the operator
/// from the bottom of the column.
///
/// We need to be able to parse the values in the column either row-wise or
/// column-wise depending on which part we are solving for. Keeping them as
/// strings makes the padding easier to deal with.
struct Column {
    values: Vec<String>,
    operator: Operator,
}

impl Column {
    fn reduce_row_wise(&self) -> usize {
        let reducer = self.operator.to_reduce_fn();
        self.values
            .iter()
            .flat_map(|string| string.trim().parse::<usize>())
            .reduce(reducer)
            .unwrap_or(0)
    }

    /// Read the values in the columns from right-to-left and return them as a vector.
    ///
    /// For example:
    /// ```
    /// 123
    ///  45
    ///   6
    /// ```
    /// would return:
    /// ```
    /// [356, 24, 1]
    /// ```
    ///
    fn extract_columnar_values(&self) -> Vec<usize> {
        // Ensure that we get the number with the most digits so we don't
        // index out of bounds.
        let longest = self.values.iter().map(String::len).max().unwrap();

        // Parse each of the rows top to bottom, right to left. These get stored into
        // the vector in reverse order, so we need to handle that before returning.
        // Example:
        // 123
        //  45
        //   6
        // Gets pushed as: [1, 24, 356]
        let mut columnar_values = vec![0; longest];
        for row in &self.values {
            for (i, c) in row.as_bytes().iter().enumerate().rev() {
                if !c.is_ascii_digit() {
                    continue;
                }

                columnar_values[i] *= 10;
                columnar_values[i] += (c - b'0') as usize;
            }
        }

        // Ensure the right-to-left order is maintained for correctness when reducing
        columnar_values.reverse();
        columnar_values
    }

    fn reduce_column_wise(&self) -> usize {
        let reducer = self.operator.to_reduce_fn();
        self.extract_columnar_values()
            .iter()
            .copied()
            .reduce(reducer)
            .unwrap_or(0)
    }
}

pub struct Day6 {
    columns: Vec<Column>,
}

impl Solution<usize, usize> for Day6 {
    fn part1(&self) -> usize {
        self.columns.iter().map(Column::reduce_row_wise).sum()
    }

    fn part2(&self) -> usize {
        self.columns.iter().map(Column::reduce_column_wise).sum()
    }
}

impl From<Input> for Day6 {
    fn from(value: Input) -> Self {
        let string = std::fs::read_to_string(value.to_string()).unwrap();
        let lines = string.lines().collect::<Vec<&str>>();

        let mut columns = vec![];
        let mut starting_column = 0;
        for (i, ch) in lines[0].chars().enumerate() {
            if ch != ' ' {
                continue;
            }

            // We only want to consider "chunks" that are delimited by all spaces in the column
            // because this is what breaks up the individual problems.
            let all_are_space = lines.iter().skip(1).all(|line| line.as_bytes()[i] == b' ');
            if !all_are_space {
                continue;
            }

            // Get all values in the current problem between the starting and ending column.
            let values = lines[0..lines.len() - 1]
                .iter()
                .map(|line| line[starting_column..i].to_string())
                .collect();

            // Get just the operator, this is always in the last row, first column
            let operator =
                Operator::try_from(&lines[lines.len() - 1][starting_column..=starting_column])
                    .unwrap();

            columns.push(Column { values, operator });
            starting_column = i + 1;
        }

        // Account for the last column which won't have an ending space.
        {
            let values = lines[0..lines.len() - 1]
                .iter()
                .map(|line| line[starting_column..].to_string())
                .collect();
            let operator = Operator::try_from(&lines[lines.len() - 1][starting_column..]).unwrap();
            columns.push(Column { values, operator });
        }

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
        assert_eq!(11_950_004_808_442, 11_950_004_808_442_usize);
    }
}
