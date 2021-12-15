use advent_of_code::day09::parse_input;
use advent_of_code::day15::{find_min_risk, map5};
mod tools;

#[test]
fn test_day15_p1() {
    let input = tools::import_test_input(15);
    let map = parse_input(input);
    let result = find_min_risk(10, &map);
    let expected: u64 = 40;
    assert_eq!(expected, result);
}

#[test]
fn test_day15_p2() {
    let input = tools::import_test_input(15);
    let map = parse_input(input);
    let map5 = map5(10, map);
    let result = find_min_risk(50, &map5);
    let expected: u64 = 315;
    assert_eq!(expected, result);
}
