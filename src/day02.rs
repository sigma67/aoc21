use crate::submarine::Submarine;

pub fn part1(input: String) -> u64 {
    let lines = input.lines();
    let mut sub = Submarine::new();
    for line in lines {
        sub.move_sub(line);
    }
    (sub.get_position() * sub.get_depth()) as u64
}

pub fn part2(input: String) -> u64 {
    let lines = input.lines();
    let mut sub = Submarine::new();
    for line in lines {
        sub.move_sub_aim(line);
    }
    (sub.get_position() * sub.get_depth()) as u64
}
