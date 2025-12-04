pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait Solution<P1, P2>
where
    P1: Display,
    P2: Display,
{
    fn part1(&self) -> P1;
    fn part2(&self) -> P2;

    fn report(&self, day: u8) {
        println!("Day {day}");
        println!("Part 1: {}", self.part1());
        println!("Part 2: {}", self.part2());
    }
}

type Day = u8;

pub enum Input {
    Sample(Day),
    Part1(Day),
    Part2(Day),
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Sample(day) => write!(f, "inputs/{day}/input.sample"),
            Input::Part1(day) => write!(f, "inputs/{day}/input.part1"),
            Input::Part2(day) => write!(f, "inputs/{day}/input.part2"),
        }
    }
}

fn read_to_iter(input: &Input) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(input.to_string())?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(Result::unwrap))
}
