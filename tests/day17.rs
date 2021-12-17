use advent_of_code::day17::shoot;
mod tools;

#[test]
fn test_day17_p1() {
    let input = tools::import_test_input(17);
    let result = shoot(&input, false);
    let expected: u64 = 45;
    assert_eq!(expected, result);
}

#[test]
fn test_day17_p2() {
    let input = tools::import_test_input(17);
    let result = shoot(&input, true);
    let expected: u64 = 112;
    assert_eq!(expected, result);
}

