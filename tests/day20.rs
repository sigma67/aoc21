use advent_of_code::day20::enhance;
mod tools;

#[test]
fn test_day20_p1() {
    let input = tools::import_test_input(20);
    assert_eq!(35, enhance(input, 5, 2));
}

#[test]
fn test_day20_p1_2() {
    let input = tools::import_test_input_2(20);
    assert_eq!(5326, enhance(input, 100, 2));
}

#[test]
fn test_day20_p2() {
    let input = tools::import_test_input(20);
    assert_eq!(3351, enhance(input, 5, 50));
}