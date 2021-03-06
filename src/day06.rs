use crate::helpers;

pub fn part1(input: String) -> u64 {
    populate_laternfish(&input, 80)
}

pub fn part2(input: String) -> u64 {
    populate_laternfish(&input, 256)
}

pub fn populate_laternfish(input: &String, days: u16) -> u64 {
    let population: Vec<u8> = input.lines().next().unwrap().split(',')
        .map(|c| c.parse::<u8>().unwrap()).collect();

    //each index i contains the number of fish with i remaining days
    let mut fishes: [u64;9] = [0; 9];

    //count the fishes
    for fish in &population{
        fishes[*fish as usize] += 1;
    }
    let mut idx:u8 = 0;
    for day in 0..days {
        let baby_fish:u64 = fishes[idx as usize];
        fishes[helpers::sub_modular(idx, 2, 9)] += baby_fish;
        fishes[idx as usize] = baby_fish;
        idx = helpers::add_modular(idx, 1, 9);
        println!("Day {:?}, {:?} total fish", day, fishes.iter().sum::<u64>());
    }

    fishes.iter().sum::<u64>()
}
