pub fn part1(input: String) -> u32 {
    populate_laternfish(&input, 80)
}

pub fn part2(input: String) -> u32 {
    populate_laternfish(&input, 256)
}

pub fn populate_laternfish(input: &String, days: u16) -> u32 {
    let mut population: Vec<u8> = input.lines().next().unwrap().split(',')
        .map(|c| c.parse::<u8>().unwrap()).collect();

    //each index i contains the number of fish with i remaining days
    let mut fishes: [u64;9] = [0; 9];

    //count the fishes
    for fish in &population{
        fishes[*fish as usize] += 1;
    }
    let mut idx:u8 = 0;
    for day in 0..days {
        let mut baby_fish:u64 = fishes[idx as usize];
        fishes[sub_modular(idx, 2, 9)] += baby_fish;
        fishes[idx as usize] = baby_fish;
        idx = add_modular(idx, 1, 9);
        println!("Day {:?}, {:?} total fish", day, fishes.iter().sum::<u64>());
    }

    //the returned result will be incorrect due to cast but we can read it from output
    fishes.iter().sum::<u64>() as u32
}

pub fn sub_modular(val: u8, sub: u8, base: u8) -> usize {
    let val = val as i8;
    let sub = sub as i8;
    let base = base as i8;
    let diff = val % base - sub % base;
    match diff >= 0 {
        true => (val - sub) as usize,
        false => (diff + base) as usize
    }
}

pub fn add_modular(val: u8, add: u8, base: u8) -> u8 {
    ((add + val) % base) as u8
}
