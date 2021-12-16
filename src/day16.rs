use std::iter::Peekable;
use std::vec::IntoIter;

pub fn part1(input: String) -> u64 {
    decode(&input, true)
}

pub fn part2(input: String) -> u64 {
    decode(&input, false)
}

pub fn decode(input: &str, version: bool) -> u64{
    let hex = hex_to_bits(&input.trim());
    let mut iter = hex.into_iter().peekable();
    let mut version_sum = 0;
    let mut depth = 0;
    let mut depth_mode: Vec<i8> = vec![-1; input.len() % 100];
    let mut depth_remaining: Vec<u64> = vec![0; input.len() % 100];
    let i = iter.by_ref();
    let mut ops:Vec<u8> = Vec::new();
    let mut vals:Vec<Vec<u64>> = vec![Vec::new(); input.len() % 100];
    while i.peek().is_some(){
        let version = to_num(take_n(i, 3));
        version_sum += version;
        let type_id = to_num(take_n(i, 3)) as u8;
        if type_id == 4 {
            let mut four = take_n(i, 5);
            let mut bit_count = 11;
            let mut literal:Vec<u8> = Vec::new();
            while four[0] == 1 {
                literal.extend(&four[1..5]);
                four = take_n(i, 5);
                bit_count += 5;
            }
            literal.extend(&four[1..5]);
            let lit = to_num(literal);
            vals[depth].push(lit);
            reduce_remaining(depth, &depth_mode, &mut depth_remaining, bit_count);
            if depth_remaining[depth - 1] == 0 {
                let depth_index = depth_remaining.iter().enumerate()
                    .filter(|(_, x) | **x!= 0)
                    .max_by(|(i, _), (j, _)| i.cmp(j));
                let depth_new = match depth_index {
                    Some(x) => x.0 + 1,
                    None => 0
                };
                for j in 0..depth-depth_new {
                    let mut num = reduce_ops(ops.pop().unwrap(), &vals[depth - j]);
                    vals[depth - j] = Vec::new();
                    vals[depth - j - 1].push(num);
                }
                depth = depth_new;
            }
            if depth == 0 || depth_remaining.iter().sum::<u64>() == 0 { break; }
        }
        else {
            ops.push(type_id);
            let mut bit_count = 7;
            depth_mode[depth] = take_n(i, 1)[0] as i8;
            let remaining = match depth_mode[depth] {
                0 => {bit_count += 15; to_num(take_n(i, 15)) },
                _ => {bit_count += 11; to_num(take_n(i, 11)) }
            };
            reduce_remaining(depth, &depth_mode, &mut depth_remaining, bit_count);
            depth_remaining[depth] = remaining;
            depth += 1;
        }
    }
    if version { version_sum } else { vals[0][0] }
}


pub fn to_num(bits: Vec<u8>) -> u64 {
    bits.iter().fold(
        0, |gamma, &bit| (gamma << 1) ^ bit as u64
    )
}

pub fn hex_to_bits(input: &str) -> Vec<u8> {
    let nums = input.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap()).collect::<Vec<_>>();
    let mut bit_vec:Vec<u8> = Vec::new();
    for num in nums {
        bit_vec.extend(
            format!("{:04b}", num).chars().map(|c| c.to_digit(2).unwrap() as u8).collect::<Vec<u8>>()
        )
    }
    bit_vec
}

fn take_n(iter: &mut Peekable<IntoIter<u8>>, count: usize) -> Vec<u8> {
    iter.take(count).collect::<Vec<_>>()
}

fn reduce_remaining(depth: usize, depth_mode: &Vec<i8>, depth_remaining: &mut Vec<u64>, by: u64){
    for i in 0..depth {
        if depth_mode[i] == 0 { depth_remaining[i] -= by; }
    }
    if depth > 0 && depth_mode[depth - 1] == 1 && depth_remaining[depth - 1] > 0 { depth_remaining[depth - 1] -= 1; }
}

fn reduce_ops(type_id: u8, vals: &Vec<u64>) -> u64 {
    match type_id {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => if vals[0] > vals[1] { 1 } else {0},
        6 => if vals[0] < vals[1] { 1 } else {0},
        7 => if vals[0] == vals[1]{ 1 } else {0},
        _ => panic!("that's not right")
    }
}