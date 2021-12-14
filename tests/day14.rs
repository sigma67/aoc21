use advent_of_code::day14::expand;
mod tools;

#[test]
fn test_day14_p1() {
    let input = tools::import_test_input(14);
    let result = expand(input, 10);
    let expected: u64 = 1588;
    assert_eq!(expected, result);
}

#[test]
fn test_day14_p2() {
    let input = tools::import_test_input(14);
    let result = expand(input, 40);
    let expected: u64 = 2188189693529;
    assert_eq!(expected, result);
}
