// Days
pub mod day01;

pub fn noop(_inp: String) -> u32 { return 0 }

pub type DayFn = fn(String) -> u32;

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
