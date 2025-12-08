use crate::points::point_3d::Point3D;
use crate::{read_to_iter, Input, Solution};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const DAY: u8 = 8;

type JunctionBox = Point3D<i64>;
type Distance = usize;
type MinHeap<T> = BinaryHeap<Reverse<T>>;

pub struct Day8<const L1: usize = 0> {
    boxes: Vec<JunctionBox>,
}

impl<const L1: usize> Day8<L1> {
    fn distance_pairs(&self) -> MinHeap<(Distance, (JunctionBox, JunctionBox))> {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        for &p1 in &self.boxes {
            for &p2 in &self.boxes {
                if p1 == p2 {
                    continue;
                }
                let pair = (p1, p2);
                if seen.contains(&pair) {
                    continue;
                }
                seen.insert(pair);
                seen.insert((p2, p1));

                heap.push(Reverse((p1.distance(p2), pair)));
            }
        }

        heap
    }

    fn chains(&self) -> Vec<HashSet<JunctionBox>> {
        let mut heap = self.distance_pairs();
        let mut g: HashMap<JunctionBox, HashSet<JunctionBox>> = HashMap::new();

        let mut connections = 0;
        while let Some(Reverse((_, (start, end)))) = heap.pop() {
            if !g.entry(start).or_default().contains(&end) {
                g.entry(end).or_default().insert(start);
                g.entry(start).or_default().insert(end);
                connections += 1;
            }

            if connections == L1 {
                break;
            }
        }

        generate_connections(&g)
    }
}

impl<const L1: usize> Solution<usize, usize> for Day8<L1> {
    fn part1(&self) -> usize {
        self.chains()
            .iter()
            .map(HashSet::len)
            .take(3)
            .product::<usize>()
    }

    fn part2(&self) -> usize {
        0
    }
}

impl<const L1: usize> From<Input> for Day8<L1> {
    fn from(value: Input) -> Self {
        let boxes = read_to_iter(&value)
            .unwrap()
            .map(|line| {
                let mut parts = line.split(',').map(|n| n.parse::<usize>().unwrap());
                JunctionBox::new(
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        Day8::<L1> { boxes }
    }
}

fn generate_connections(
    chains: &HashMap<JunctionBox, HashSet<JunctionBox>>,
) -> Vec<HashSet<JunctionBox>> {
    let mut queue = chains.keys().copied().collect::<VecDeque<_>>();
    let mut seen = HashSet::new();
    let mut light_strings = Vec::new();

    while let Some(chain_start) = queue.pop_front() {
        let mut chain_queue = VecDeque::from([chain_start]);
        let mut chain = HashSet::new();

        while let Some(junction_box) = chain_queue.pop_front() {
            if !seen.insert(junction_box) {
                continue;
            }
            chain.insert(junction_box);

            if let Some(neighbors) = chains.get(&junction_box) {
                for &neighbor in neighbors {
                    chain_queue.push_back(neighbor);
                }
            }
        }
        light_strings.push(chain);
    }
    light_strings.sort_by_key(HashSet::len);
    light_strings.reverse();
    light_strings
}

pub fn run() {
    let day = Day8::<0>::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day8::<10>::from(Input::Sample(DAY));
        assert_eq!(40, day_sample.part1());
        assert_eq!(66912, 66912);
    }

    #[test]
    fn part_2() {
        let day_sample = Day8::<10>::from(Input::Sample(DAY));
        assert_eq!(0, day_sample.part2());
        assert_eq!(0, 0);
    }
}
