#[macro_use]
extern crate criterion;

use advent_of_code::day_1::Day1;
use advent_of_code::day_2::Day2;
use advent_of_code::day_3::Day3;
use advent_of_code::{Input, Solution};
use criterion::{criterion_main, Criterion};
use std::fmt::Display;
use std::time::Duration;

fn helper<F, P1, P2>(c: &mut Criterion, name: &str, generate_fn: F)
where
    F: Fn() -> Box<dyn Solution<P1, P2>>,
    P1: Display,
    P2: Display,
{
    let mut group = c.benchmark_group(name);
    group
        .sample_size(100)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10));

    let solution = generate_fn();
    group.bench_function("parse", |b| {
        b.iter(|| std::hint::black_box(generate_fn()));
    });
    group.bench_function("part 1", |b| {
        b.iter(|| std::hint::black_box(solution.part1()));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| std::hint::black_box(solution.part2()));
    });
    group.finish();
}

fn all_days(c: &mut Criterion) {
    helper(c, "day 1", || Box::new(Day1::from(Input::Part1(1))));
    helper(c, "day 2", || Box::new(Day2::from(Input::Part1(2))));
    helper(c, "day 3", || Box::new(Day3::from(Input::Part1(3))));
}

criterion_group!(days, all_days);
criterion_main!(days);
