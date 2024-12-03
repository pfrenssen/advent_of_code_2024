use aoc_runner_derive::aoc_lib;

mod day1;
mod day2;
mod day3;

aoc_lib! { year = 2024 }

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}
