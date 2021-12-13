use advent_of_code::day13::fold;
mod tools;

#[test]
fn test_day13_p1() {
    let input = tools::import_test_input(13);
    let result = fold(input, 1);
    let expected: u64 = 17;
    assert_eq!(expected, result);
}
