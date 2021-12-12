use advent_of_code::day12::count_paths;
mod tools;

#[test]
fn test_day12_p1() {
    let input = tools::import_test_input(12);
    let result = count_paths(input, false);
    let expected: u64 = 19;
    assert_eq!(expected, result);
    let input = tools::import_test_input_2(12);
    let result = count_paths(input, false);
    let expected: u64 = 226;
    assert_eq!(expected, result);
}

#[test]
fn test_day12_p2() {
    let input = tools::import_test_input(12);
    let result = count_paths(input, true);
    let expected: u64 = 103;
    assert_eq!(expected, result);
    let input = tools::import_test_input_2(12);
    let result = count_paths(input, true);
    let expected: u64 = 3509;
    assert_eq!(expected, result);
}