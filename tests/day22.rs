use advent_of_code::day22::{part1, Cuboid, intersection_range};
mod tools;
use itertools::Itertools;

#[test]
fn test_day22_p1() {
    let input = tools::import_test_input(22);
    assert_eq!(39, part1(input));
}

#[test]
fn test_day22_p1_2() {
    let input = tools::import_test_input_2(22);
    assert_eq!(590784, part1(input));
}

#[test]
pub fn test_intersection_range() {
    assert_eq!(Some((3,5)), intersection_range((1,5), (3,8)));
    assert_eq!(Some((3,5)), intersection_range((3,8), (1,5)));
    assert_eq!(Some((-35,5)), intersection_range((-35,5), (-77,20)));
    assert_eq!(Some((-30,1)), intersection_range((-35,5), (-30,1)));
    assert_eq!(Some((5,5)), intersection_range((1,5), (5,8)));
    assert_eq!(None, intersection_range((1,5), (6,8)));
}