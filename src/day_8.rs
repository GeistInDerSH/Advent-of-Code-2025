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
        (0..self.boxes.len())
            .flat_map(|a| (a + 1..self.boxes.len()).map(move |b| (a, b)))
            .map(|(a, b)| {
                let p1 = self.boxes[a];
                let p2 = self.boxes[b];
                Reverse((p1.distance(p2), (p1, p2)))
            })
            .collect::<MinHeap<_>>()
    }
}

impl<const L1: usize> Solution<usize, usize> for Day8<L1> {
    fn part1(&self) -> usize {
        let mut heap = self.distance_pairs();
        let mut connection_map: HashMap<JunctionBox, HashSet<JunctionBox>> = HashMap::new();

        let mut connections = 0;
        while let Some(Reverse((_, (start, end)))) = heap.pop() {
            if !connection_map.entry(start).or_default().contains(&end) {
                connection_map.entry(end).or_default().insert(start);
                connection_map.entry(start).or_default().insert(end);
                connections += 1;
            }

            if connections == L1 {
                break;
            }
        }

        generate_connections(&connection_map)
            .iter()
            .take(3)
            .map(HashSet::len)
            .product()
    }

    fn part2(&self) -> usize {
        let mut heap = self.distance_pairs();

        let mut connections: Vec<HashSet<JunctionBox>> = Vec::new();
        while let Some(Reverse((_, (start, end)))) = heap.pop() {
            let start_index = connections.iter().position(|chain| chain.contains(&start));
            let end_index = connections.iter().position(|chain| chain.contains(&end));

            let circuit = match (start_index, end_index) {
                (None, None) => {
                    connections.push(HashSet::from([start, end]));
                    None
                }
                (Some(l1), None) => {
                    connections[l1].insert(end);
                    Some(l1)
                }
                (None, Some(l2)) => {
                    connections[l2].insert(start);
                    Some(l2)
                }
                (Some(l1), Some(l2)) => {
                    if l1 == l2 {
                        None
                    } else {
                        let add_to = l1.min(l2);
                        let remove_from = l1.max(l2);
                        let removed = connections.remove(remove_from);
                        connections[add_to].extend(removed);
                        Some(add_to)
                    }
                }
            };

            if let Some(location) = circuit
                && connections[location].len() == self.boxes.len()
            {
                return usize::try_from(start.x * end.x).unwrap_or(0);
            }
        }

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
    let mut queue = chains.keys().collect::<BinaryHeap<_>>();
    let mut seen = HashSet::new();
    let mut light_strings = Vec::new();

    while let Some(chain_start) = queue.pop() {
        if seen.contains(chain_start) {
            continue;
        }

        let mut chain_queue = VecDeque::from([chain_start]);
        let mut chain = HashSet::new();

        while let Some(junction_box) = chain_queue.pop_front() {
            if !seen.insert(junction_box) {
                continue;
            }
            chain.insert(*junction_box);

            if let Some(neighbors) = chains.get(junction_box) {
                for neighbor in neighbors {
                    chain_queue.push_back(neighbor);
                }
            }
        }

        if !chain.is_empty() {
            light_strings.push(chain);
        }
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
        assert_eq!(25272, day_sample.part2());
        assert_eq!(724_454_082, 724_454_082);
    }
}
