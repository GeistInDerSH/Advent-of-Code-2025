use crate::{read_to_iter, Input, Solution};
use std::collections::HashSet;

const DAY: u8 = 5;

type RangeStart = usize;
type RangeEnd = usize;
type FreshIngredients = (RangeStart, RangeEnd);
type Ingredient = usize;

pub struct Day5 {
    fresh_ids: Vec<FreshIngredients>,
    ingredient_ids: Vec<Ingredient>,
}

impl Solution<usize, usize> for Day5 {
    fn part1(&self) -> usize {
        let mut fresh_ingredients = HashSet::new();
        for &ingredient in &self.ingredient_ids {
            for range in &self.fresh_ids {
                if is_within_range(ingredient, range) {
                    fresh_ingredients.insert(ingredient);
                }
            }
        }

        fresh_ingredients.len()
    }

    fn part2(&self) -> usize {
        // Sort the fresh ids by the start of the range so that we can combine ranges more
        // easily later on without needing to do multiple passes.
        let sorted_fresh_ids = {
            let mut ids = self.fresh_ids.clone();
            ids.sort_by(|a, b| a.0.cmp(&b.0));
            ids
        };

        let mut valid_id_ranges: Vec<FreshIngredients> = Vec::new();
        for fresh_range in &sorted_fresh_ids {
            let mut added = false;

            // Check to see if any of the FreshIngredients ranges we've added has
            // an overlap with the current range. If it does, we'll modify that range
            // so it includes the current one.
            // If we don't find it, we'll add it as a new range.
            for entry in &mut valid_id_ranges {
                if has_overlap(*fresh_range, *entry) {
                    *entry = intersection(*fresh_range, *entry);
                    added = true;
                }
            }

            if !added {
                valid_id_ranges.push(*fresh_range);
            }
        }

        valid_id_ranges
            .iter()
            .fold(0, |acc, (start, end)| acc + (end - start) + 1)
    }
}

impl From<Input> for Day5 {
    fn from(value: Input) -> Self {
        let iter = read_to_iter(&value).unwrap().collect::<Vec<String>>();
        let fresh_ids = iter
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (lhs, rhs) = line.split_once('-').unwrap();
                (lhs.parse().unwrap(), rhs.parse().unwrap())
            })
            .collect::<Vec<_>>();
        let ingredient_ids = iter
            .iter()
            .skip(fresh_ids.len() + 1)
            .map(|line| line.parse().unwrap())
            .collect();

        Day5 {
            fresh_ids,
            ingredient_ids,
        }
    }
}

/// Check to see if the given [`Ingredient`] is within the range.
fn is_within_range(value: Ingredient, range: &FreshIngredients) -> bool {
    range.0 < value && value <= range.1
}

/// Check if the two [`FreshIngredients`] contain any overlap in the values
/// within their bounds.
fn has_overlap(r1: FreshIngredients, r2: FreshIngredients) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}

/// Find the intersection of the two [`FreshIngredients`] ranges and return
/// the resulting range.
fn intersection(r1: FreshIngredients, r2: FreshIngredients) -> FreshIngredients {
    (r1.0.min(r2.0), r1.1.max(r2.1))
}

pub fn run() {
    let day = Day5::from(Input::Sample(DAY));
    day.report(DAY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_sample = Day5::from(Input::Sample(DAY));
        assert_eq!(3, day_sample.part1());
        assert_eq!(638, 638);
    }

    #[test]
    fn part_2() {
        let day_sample = Day5::from(Input::Sample(DAY));
        assert_eq!(14, day_sample.part2());
        assert_eq!(352_946_349_407_338, 352_946_349_407_338_usize);
    }
}
