mod drop_at;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::time::{Duration, Instant};

macro_rules! input {
    ($day:ident) => {
        include_str!(concat!("input/", stringify!($day), ".txt"))
    };
}

macro_rules! run {
    ($day:ident) => {
        let input = input!($day);
        println!("{}:", stringify!($day));
        println!("\tPart1: {:?}", time(|| $day::part1(input)));
        println!("\tPart2: {:?}", time(|| $day::part2(input)));
    };
}

fn main() {
    run!(day1);
    run!(day2);
    run!(day3);
    run!(day4);
    run!(day5);
    run!(day6);
    run!(day7);
    run!(day8);
}

fn time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let now = Instant::now();
    let result = f();
    (result, now.elapsed())
}
