use advent_of_code::day23::play_game;
mod tools;

#[test]
fn test_day23_p1() {
    let input = tools::import_test_input(23);
    assert_eq!(12521, play_game(input, 2));
}

#[test]
fn test_day23_p1_2() {
    let input = tools::import_test_input_2(23);
    assert_eq!(44169, play_game(input, 4));
}