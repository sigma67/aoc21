use advent_of_code::day21::{play, play_quantum, parse_input};
use advent_of_code::helpers::add_modular_no0;
mod tools;
use itertools::Itertools;

#[test]
fn test_day21_p1() {
    let input = tools::import_test_input(21);
    assert_eq!(739785, play(parse_input(input), 1000));
}

#[test]
fn test_day21_p2() {
    let input = tools::import_test_input(21);
    assert_eq!(444356092776315, play_quantum(parse_input(input)));
}

#[test]
fn test_add_modular() {
    for i in 0..202 {
        println!("{} {}", i + 1, add_modular_no0(i, 1, 10))
    }
    assert_eq!(1, 1);
}