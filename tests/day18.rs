use advent_of_code::day18::{parse_line, add_snailfish, reduce, magnitude};
mod tools;

#[test]
fn test_day18_p1() {
    let input = tools::import_test_input(18);
    assert_eq!(3488, add_snailfish(&mut parse_lines(input)));
    let input = tools::import_test_input_2(18);
    assert_eq!(4140, add_snailfish(&mut parse_lines(input)));
}

#[test]
fn test_reduce() {
    let input = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
    let mut vals = parse_line(input);
    reduce(&mut vals);
    assert_eq!(1384, magnitude(&mut vals));
}

#[test]
fn test_magnitude() {
    let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
    assert_eq!(1384, magnitude(&mut parse_line(input)));
    let input = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
    assert_eq!(791, magnitude(&mut parse_line(input)));
    let input = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
    assert_eq!(1137, magnitude(&mut parse_line(input)));
    let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
    assert_eq!(3488, magnitude(&mut parse_line(input)));
}