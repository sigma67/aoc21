use advent_of_code::day19::get_beacons;
mod tools;

#[test]
fn test_day19_p1() {
    let input = tools::import_test_input(19);
    assert_eq!(79, get_beacons(input, false));
}

#[test]
fn test_day19_p2() {
    let input = tools::import_test_input(19);
    assert_eq!(3621, get_beacons(input, true));
}

