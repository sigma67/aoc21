use advent_of_code::day23::{part1};
mod tools;

#[test]
fn test_day23_p1() {
    let input = tools::import_test_input(23);
    assert_eq!(39, part1(input));
}

#[test]
fn test_day23_p1_2() {
    let input = tools::import_test_input_2(23);
    assert_eq!(590784, part1(input));
}