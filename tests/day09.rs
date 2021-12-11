use advent_of_code::day09::{find_basins, find_hotspots};
mod tools;


#[test]
fn test_day09_p1() {
    let input = tools::import_test_input(9);
    let result = find_hotspots(input, 10);
    let expected: u64 = 15;
    assert_eq!(expected, result);
}

#[test]
fn test_day09_p2() {
    let input = tools::import_test_input(9);
    let result = find_basins(input, 10);
    let expected: u64 = 1134;
    assert_eq!(expected, result);
}
