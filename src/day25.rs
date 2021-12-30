pub fn part1(input: String) -> u64 {
    step(input)
}

pub fn part2(input: String) -> u64 {
    step(input)
}

pub fn step(input: String) -> u64 {
    let mut map : Vec<u8> = Vec::new();
    let mut xlen = 0;
    for line in input.lines() {
        xlen = line.len();
        map.extend(line.chars().map(|c| match c {
            '>' => 1,
            'v' => 2,
            _ => 0
        }));
    }
    let mut steps = 1;
    loop {
        let s1 = update(map, 1, xlen);
        let s2 = update(s1.0, 2, xlen);
        map = s2.0;
        if s1.1 + s2.1 == 0 { break; }
        steps +=1;
    }
    steps
}

fn update(map: Vec<u8>, kind: u8, xlen: usize) -> (Vec<u8>, usize) {
    let mut new_map = map.clone();
    let mut moves = 0;
    for i in 0..map.len() {
        let n = match map[i] {
            1 if kind == 1 => (i + 1) % xlen + (i / xlen * xlen),
            2 if kind == 2 => (i + xlen) % map.len(),
            _ => continue
        };
        if map[n] != 0 { continue }
        new_map[n] = map[i];
        new_map[i] = 0;
        moves += 1;
    }
    (new_map, moves)
}