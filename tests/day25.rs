use advent_of_code::day25::step;
mod tools;

#[test]
fn test_day25_p1() {
    let input = tools::import_test_input(25);
    assert_eq!(58, step(input));
}
