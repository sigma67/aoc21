use advent_of_code::day07::find_optimal_position;

#[test]
fn test_day07(){
    let input: String = String::from("16,1,2,0,4,2,7,1,2,14");
    let result = find_optimal_position(&input, false);
    let expected: u64 = 37;
    assert_eq!(expected, result);
    let result = find_optimal_position(&input, true);
    let expected: u64 = 168;
    assert_eq!(expected, result);
}