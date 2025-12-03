use crate::{read_to_iter, Input, Solution};

const DAY: u8 = 3;

type Battery = Vec<usize>;
type BatteryBanks = Vec<Battery>;

pub struct Day3(BatteryBanks);

impl Solution<usize, usize> for Day3 {
    fn part1(&self) -> usize {
        self.0
            .iter()
            .map(|battery| {
                let (index, largest) = battery
                    .iter()
                    .take(battery.len() - 1) // skip the last element to ensure there is at least one left over
                    .enumerate()
                    .rev() // reverse so max_by will return the first element that is smallest
                    .max_by(|a, b| a.1.cmp(b.1))
                    .unwrap_or((battery[0], &0));
                let next_largest = battery.iter().skip(index + 1).max().unwrap_or(&0);
                largest * 10 + next_largest
            })
            .sum()
    }

    fn part2(&self) -> usize {
        0
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
        assert_eq!(0, day_sample.part2());
        assert_eq!(0, 0);
    }
}
