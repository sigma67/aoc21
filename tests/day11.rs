use advent_of_code::day11::count_flashes;
mod tools;

#[test]
fn test_day11_p1() {
    let input = tools::import_test_input(11);
    let result = count_flashes(input, 100);
    let expected: u64 = 1656;
    assert_eq!(expected, result);
}

#[test]
fn test_day11_p2() {
    let input = tools::import_test_input(11);
    let result = count_flashes(input, 200);
    let expected: u64 = 195;
    assert_eq!(expected, result);
}