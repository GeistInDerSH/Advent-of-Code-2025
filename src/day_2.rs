use crate::{helper, Input, Solution};

const DAY: u8 = 2;

type Ranges = Vec<(usize, usize)>;

pub struct Day2(Ranges);

impl Day2 {
    fn solution(&self, filter_fn: fn(usize) -> bool) -> usize {
        self.0
            .iter()
            .map(|&(start, end)| {
                (start..=end)
                    .filter(|&value| filter_fn(value))
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Solution<usize, usize> for Day2 {
    fn part1(&self) -> usize {
        self.solution(has_repetition_twice)
    }

    fn part2(&self) -> usize {
        self.solution(has_repetition_at_least_twice)
    }
}

impl From<Input> for Day2 {
    fn from(value: Input) -> Self {
        let contents = std::fs::read_to_string(value.to_string()).unwrap();
        let ranges = contents
            .split(',')
            .map(helper::parsing::line_to_range_tuple::<&str, usize>)
            .collect::<Vec<_>>();
        Day2(ranges)
    }
}

fn has_repetition_twice(value: usize) -> bool {
    if !(value.checked_ilog10().unwrap_or(0) + 1).is_multiple_of(2) {
        return false;
    }
    let string = value.to_string();
    let (lhs, rhs) = string.split_at(string.len() / 2);
    lhs == rhs
}

fn has_repetition_at_least_twice(value: usize) -> bool {
    let string = value.to_string();
    let bytes = string.as_bytes();
    // shortcut for all being the same char
    if bytes.iter().all(|&b| b == bytes[0]) {
        return true;
    }

    let length = bytes.len();
    (2..=length / 2)
        .filter(|&sub_len| length.is_multiple_of(sub_len))
        .any(|sub_len| {
            let mut seq = bytes.chunks(sub_len);
            let first = seq.next().unwrap();
            seq.all(|chunk| chunk == first)
        })
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
        assert_eq!(37_314_786_486_usize, 37_314_786_486_usize);
    }

    #[test]
    fn part_2() {
        let day_sample = Day2::from(Input::Sample(DAY));
        assert_eq!(4_174_379_265, day_sample.part2());
        assert_eq!(47_477_054_026, 47_477_054_026usize);
    }
}
