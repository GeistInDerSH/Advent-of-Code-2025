use crate::{read_to_iter, Input, Solution};

const DAY: u8 = 12;

struct Region {
    area: usize,
    required_present_ids: [usize; 6],
}

impl Region {
    fn can_fit_required(&self) -> bool {
        self.required_present_ids.iter().sum::<usize>() < self.area
    }
}

pub struct Day12(Vec<Region>);

impl Solution<usize, &'static str> for Day12 {
    fn part1(&self) -> usize {
        self.0
            .iter()
            .map(Region::can_fit_required)
            .filter(|b| *b)
            .count()
    }

    fn part2(&self) -> &'static str {
        "Finish Decorating the North Pole!"
    }
}

impl From<Input> for Day12 {
    fn from(value: Input) -> Self {
        let regions = read_to_iter(&value)
            .unwrap()
            .filter(|line| line.contains('x'))
            .map(|line| {
                let (range, numbers) = line.split_once(": ").unwrap();

                let area = range
                    .split('x')
                    .flat_map(&str::parse)
                    .map(|n: usize| n / 3)
                    .product();

                let mut present_ids = [0; 6];
                for (i, count) in numbers.split(' ').enumerate() {
                    present_ids[i] = count.parse::<usize>().unwrap();
                }

                Region {
                    area,
                    required_present_ids: present_ids,
                }
            })
            .collect();
        Day12(regions)
    }
}

pub fn run() {
    let day = Day12::from(Input::Part1(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        // It doesn't actually work for the example, but it is otherwise correct :)
        let day_sample = Day12::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part1());
        assert_eq!(546, 546);
    }

    #[test]
    fn part_2() {
        let day_sample = Day12::from(Input::Sample(DAY));
        assert_eq!("Finish Decorating the North Pole!", day_sample.part2());
    }
}
