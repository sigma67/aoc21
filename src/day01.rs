pub fn part1(input: String) -> u64 {
    let lines = input.lines();
    let mut count: u32 = 0;
    let mut prev: Option<u32> = None;
    for line in lines {
        let val: u32 = line.parse().unwrap();
        if prev == None {
            prev = Some(val);
            continue;
        }
        count += (val > prev.unwrap()) as u32;
        prev = Some(val);
    }
    count as u64
}

pub fn part2(input: String) -> u64 {
    let lines = input.lines();
    let mut window: [u32; 3] = [0; 3];
    let mut count: u32 = 0;
    let mut sum: u32 = 0;
    for (i, line) in lines.enumerate() {
        let val: u32 = line.parse().unwrap();
        window[i % 3] = val;
        let sum_current = window.iter().sum();
        if i < 3 { // we only have values to compare after 4 iterations
            if i == 2 { sum = sum_current }
            continue;
        }
        count += (sum_current > sum) as u32;
        sum = sum_current;
    }
    count as u64
}
