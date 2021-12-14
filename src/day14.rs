use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    expand(input, 10)
}

pub fn part2(input: String) -> u64 {
    expand(input, 40)
}

pub fn expand(input: String, rounds: u8) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let poly = lines[0].as_bytes();
    let mut insert_map = HashMap::new();
    let mut combo_counts = HashMap::new();
    let mut combos = Vec::new();
    for line in &lines[2..lines.len()] {
        let (combo, result) = line.split_once(" -> ").unwrap();
        let key = to_key(combo.as_bytes());
        insert_map.insert(key, result.as_bytes()[0]);
        *combo_counts.entry(key).or_insert(0_u64);
        combos.push(combo.as_bytes());
    }

    //init
    for win in poly.windows(2){
        *combo_counts.entry(to_key(win)).or_insert(0) += 1;
    }
    let mut char_counts = HashMap::new();
    for c in poly {
        *char_counts.entry(*c).or_insert(0) += 1;
    }

    //loop
    for _ in 0..rounds {
        let mut combo_counts_new = combo_counts.clone();
        for c in &combos{
            let k = &to_key(c);
            *combo_counts_new.entry(to_key(&[c[0], insert_map[k]])).or_insert(0) += combo_counts[k];
            *combo_counts_new.entry(to_key(&[insert_map[k], c[1]])).or_insert(0) += combo_counts[k];
            *combo_counts_new.entry(*k).or_insert(0) -= combo_counts[k];
            *char_counts.entry(insert_map[k]).or_insert(0) += combo_counts[k];
        }
        combo_counts = combo_counts_new;
    }
    let min = char_counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max = char_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    max - min
}

fn to_key(input: &[u8]) -> u16 {
    let a = input[0] as u16;
    let b = input[1] as u16;
    (a + b)*(a + b + 1)/2 + b //cantor pairing function
}
