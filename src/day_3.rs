use crate::{read_to_iter, Input, Solution};

const DAY: u8 = 3;

type Battery = Vec<usize>;
type BatteryBanks = Vec<Battery>;

pub struct Day3(BatteryBanks);

impl Day3 {
    fn solve(&self, length: usize) -> usize {
        self.0
            .iter()
            .map(|battery| find_max_joltage(battery, length))
            .sum()
    }
}

impl Solution<usize, usize> for Day3 {
    fn part1(&self) -> usize {
        self.solve(2)
    }

    fn part2(&self) -> usize {
        self.solve(12)
    }
}

impl From<Input> for Day3 {
    fn from(value: Input) -> Self {
        let iter = read_to_iter(&value)
            .unwrap()
            .map(|line| {
                let bytes = line.as_bytes();
                bytes
                    .iter()
                    .map(move |b| (b - b'0') as usize)
                    .collect::<Vec<_>>()
            })
            .collect();
        Day3(iter)
    }
}

pub fn run() {
    let day = Day3::from(Input::Sample(DAY));
    day.report(DAY);
}

fn find_next_largest(slice: &[usize], offset: usize, limit: usize) -> (usize, usize) {
    let (index, &value) = slice
        .iter()
        .skip(offset)
        .take(limit)
        .enumerate()
        .rev() // reverse so max_by will return the first element that is smallest
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap_or((0, &slice[0]));
    (index, value)
}

/// Find the largest int value that can be constructed from the slice by concatenating
/// the slice values.
fn find_max_joltage(slice: &[usize], length: usize) -> usize {
    // start by finding the largest element. This element should allow for (length-1) elements to
    // be after it so we can correctly construct the return value.
    let (index, value) = find_next_largest(slice, 0, slice.len() - (length - 1));

    let mut skip_count = index + 1;
    let mut cell_count = 1;
    let mut jolts = value;
    while cell_count < length {
        // Find the next set of values where we will look for the max value.
        // This should leave enough values after to ensure that the number of
        // cells is equal to the given length
        let window = slice.len() - skip_count - (length - cell_count) + 1;
        let (index, value) = find_next_largest(slice, skip_count, window);

        skip_count += index + 1;
        cell_count += 1;
        jolts = jolts * 10 + value;
    }

    jolts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day3::from(Input::Sample(DAY));
        assert_eq!(357, day_sample.part1());
        assert_eq!(17613, 17613);
    }

    #[test]
    fn part_2() {
        let day_sample = Day3::from(Input::Sample(DAY));
        assert_eq!(3_121_910_778_619, day_sample.part2());
        assert_eq!(175_304_218_462_560, 175_304_218_462_560_usize);
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(
            987_654_321_111,
            find_max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12)
        );
        assert_eq!(
            434_234_234_278,
            find_max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12)
        );
    }
}
