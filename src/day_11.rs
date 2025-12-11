use crate::{read_to_iter, Input, Solution};
use std::collections::{HashMap, HashSet, VecDeque};

const DAY: u8 = 11;

type Device = String;
type ConnectedDevices = Vec<Device>;

type DacSeen = bool;
type FftSeen = bool;
type SeenDevice<'a> = (&'a Device, DacSeen, FftSeen);

pub struct Day11(HashMap<Device, ConnectedDevices>);

impl Day11 {
    fn traversal_path_count(&self) -> usize {
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

    fn traverse_containing_nodes<'a>(
        &'a self,
        current_device: &'a String,
        dac_seen: DacSeen,
        fft_seen: FftSeen,
        visited: &mut HashMap<SeenDevice<'a>, i64>,
    ) -> i64 {
        let entry: SeenDevice = (current_device, dac_seen, fft_seen);
        if visited[&entry] != -1 {
            return visited[&entry];
        } else if current_device == "out" {
            let value = i64::from(dac_seen && fft_seen);
            visited.insert(entry, value);
            return value;
        }

        let count = self.0[current_device]
            .iter()
            .map(|neighbor| {
                let dac_passed = dac_seen || neighbor == "dac";
                let fft_passed = fft_seen || neighbor == "fft";
                self.traverse_containing_nodes(neighbor, dac_passed, fft_passed, visited)
            })
            .sum();
        visited.insert(entry, count);
        count
    }
}

impl Solution<usize, i64> for Day11 {
    fn part1(&self) -> usize {
        self.traversal_path_count()
    }

    fn part2(&self) -> i64 {
        let mut memoized: HashMap<SeenDevice, i64> = HashMap::new();
        for (node, neighbors) in &self.0 {
            memoized.insert((node, false, false), -1);
            memoized.insert((node, true, false), -1);
            memoized.insert((node, false, true), -1);
            memoized.insert((node, true, true), -1);

            for neighbor in neighbors {
                memoized.insert((neighbor, false, false), -1);
                memoized.insert((neighbor, true, false), -1);
                memoized.insert((neighbor, false, true), -1);
                memoized.insert((neighbor, true, true), -1);
            }
        }

        self.traverse_containing_nodes(&"svr".to_string(), false, false, &mut memoized)
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
        assert_eq!(749, 749);
    }

    #[test]
    fn part_2() {
        let day_sample = Day11::from(Input::Sample2(DAY));
        assert_eq!(2, day_sample.part2());
        assert_eq!(420_257_875_695_750, 420_257_875_695_750_i64);
    }
}
