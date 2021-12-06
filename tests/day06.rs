use advent_of_code::day06::populate_laternfish;
use advent_of_code::day06::sub_modular;

#[test]
fn test_day06(){
    let input: String = String::from("3,4,3,1,2");
    let result = populate_laternfish(&input, 18);
    let expected: u32 = 26;
    assert_eq!(expected, result);
    let result = populate_laternfish(&input, 80);
    let expected: u32 = 5934;
    assert_eq!(expected, result)
}

#[test]
fn test_sub_modular(){
    println!("{}", sub_modular(2, 2, 8))
}
