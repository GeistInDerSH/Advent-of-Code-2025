use crate::{read_to_iter, Input, Solution};
use std::collections::{HashMap, HashSet, VecDeque};

const DAY: u8 = 11;

type Device = String;
type ConnectedDevices = Vec<Device>;

pub struct Day11(HashMap<Device, ConnectedDevices>);

impl Day11 {
    fn traversal_paths(&self) -> usize {
        let mut queue = VecDeque::from([("you".to_string(), 0)]);
        let mut visited = HashSet::new();

        let mut count = 0;
        while let Some((name, cost)) = queue.pop_front() {
            if name == "out" {
                count += 1;
                continue;
            }

            visited.insert(name.clone());
            let unvisited: Vec<_> = self.0[&name]
                .iter()
                .filter(|&name| !visited.contains(name))
                .map(|d| (d.clone(), cost + 1))
                .collect();
            queue.extend(unvisited);
        }

        count
    }
}

impl Solution<usize, usize> for Day11 {
    fn part1(&self) -> usize {
        for (device, connected_devices) in &self.0 {
            println!("{device} {connected_devices:?}");
        }
        self.traversal_paths()
    }

    fn part2(&self) -> usize {
        0
    }
}

impl From<Input> for Day11 {
    fn from(value: Input) -> Self {
        let server = read_to_iter(&value)
            .unwrap()
            .map(|line| {
                let (device, raw_connections) = line.split_once(": ").unwrap();
                let connections = raw_connections
                    .split(' ')
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                (device.to_string(), connections)
            })
            .collect::<HashMap<_, _>>();

        Day11(server)
    }
}

pub fn run() {
    let day = Day11::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day11::from(Input::Sample(DAY));
        assert_eq!(5, day_sample.part1());
        let day_sample = Day11::from(Input::Part1(DAY));
        assert_eq!(0, day_sample.part1());
    }

    #[test]
    fn part_2() {
        let day_sample = Day11::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part2());
        assert_eq!(0, 0);
    }
}
