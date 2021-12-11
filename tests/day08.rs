use advent_of_code::day08::{count_unique, decode_input};
mod tools;


#[test]
fn test_day08_p1() {
    let input = tools::import_test_input(8);
    let result = count_unique(input);
    let expected: u64 = 26;
    assert_eq!(expected, result);
}
#[test]
fn test_day08_p2() {
    let input = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    let result = decode_input(input);
    let expected: u64 = 5353;
    assert_eq!(expected, result);

}
