use crate::{read_to_iter, Input, Solution};

const DAY: u8 = 1;

#[derive(Debug)]
enum Dial {
    Left(i64),
    Right(i64),
}

pub struct Day1(Vec<Dial>);

impl Solution<usize, usize> for Day1 {
    fn part1(&self) -> usize {
        self.0
            .iter()
            .fold((0, 50), |(count, position), dial| {
                let new_position = match dial {
                    Dial::Left(v) => position - *v,
                    Dial::Right(v) => position + *v,
                }
                .wrapping_rem_euclid(100);
                let new_count = count + usize::from(new_position == 0);
                (new_count, new_position)
            })
            .0
    }

    fn part2(&self) -> usize {
        self.0
            .iter()
            .flat_map(|dial| match dial {
                Dial::Right(v) => (0..*v).map(|_| 1).collect::<Vec<i64>>(),
                Dial::Left(v) => (0..*v).map(|_| -1).collect(),
            })
            .fold((0, 50), |(count, position), dial| {
                let new_position = (dial + position).wrapping_rem_euclid(100);
                if new_position == 0 {
                    (count + 1, new_position)
                } else {
                    (count, new_position)
                }
            })
            .0
    }
}

impl From<Input> for Day1 {
    fn from(value: Input) -> Self {
        let instructions = read_to_iter(&value)
            .unwrap()
            .map(|line| {
                let bytes = line.trim().as_bytes();
                let num = line.trim()[1..].parse().unwrap();
                match bytes[0] {
                    b'L' => Dial::Left(num),
                    b'R' => Dial::Right(num),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();
        Day1(instructions)
    }
}

pub fn run() {
    let day = Day1::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day1::from(Input::Sample(DAY));
        assert_eq!(3, day_sample.part1());
        assert_eq!(1165, 1165);
    }

    #[test]
    fn part_2() {
        let day_sample = Day1::from(Input::Sample(DAY));
        assert_eq!(6, day_sample.part2());
        assert_eq!(6496, 6496);
    }
}
