use advent_of_code::day10::get_score;
mod tools;

#[test]
fn test_day10_p1() {
    let input = tools::import_test_input(10);
    let result = get_score(input, true);
    let expected: u32 = 26397;
    assert_eq!(expected, result);
}

#[test]
fn test_day10_p2() {
    let input = tools::import_test_input(10);
    let result = get_score(input, false);
    let expected: u32 = 288957;
    assert_eq!(expected, result);
}
