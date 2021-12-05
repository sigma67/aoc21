use advent_of_code::day05::find_overlaps;
mod tools;

#[test]
fn test_part1(){
    let input = tools::import_test_input(5);

    let expected:u32 = 12;
    let result = find_overlaps(input, 10,true);
    assert_eq!(expected, result)
}