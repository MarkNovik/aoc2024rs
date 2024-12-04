mod day1;
mod day2;
mod drop_at;
mod day3;
mod day4;

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
}

fn time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let now = Instant::now();
    let result = f();
    (result, now.elapsed())
}
