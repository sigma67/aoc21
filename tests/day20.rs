use advent_of_code::day20::enhance;
mod tools;

#[test]
fn test_day20_p1() {
    let input = tools::import_test_input(20);
    assert_eq!(35, enhance(input, 5));
}
