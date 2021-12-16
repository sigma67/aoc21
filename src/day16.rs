use std::iter::Peekable;
use std::vec::IntoIter;

struct Packet {
    version: u8,
    type_id: u8
}

pub fn part1(input: String) -> u64 {
    decode(input)
}

pub fn part2(input: String) -> u64 {
    0
}

pub fn decode(input: String) -> u64{
    let hex = hex_to_bits(&input.trim());
    let mut iter = hex.into_iter().peekable();
    let mut packets:Vec<Packet> = Vec::new();
    let mut version_sum = 0;
    let mut depth = 0;
    let mut depth_mode: Vec<i8> = vec![-1; 100];
    let mut depth_remaining: Vec<u64> = vec![0; 100];
    let i = iter.by_ref();
    while i.peek().is_some(){
        let version = to_num(take_n(i, 3));
        version_sum += version;
        let type_id = to_num(take_n(i, 3));
        let mut content;
        if type_id == 4 {
            let mut four = take_n(i, 5);
            let mut bit_count = 11;
            let mut literal:Vec<u8> = Vec::new();
            while four[0] == 1 {
                literal.extend(&four[1..4]);
                four = take_n(i, 5);
                bit_count += 5;
            }
            literal.extend(four);
            reduce_remaining(depth, &depth_mode, &mut depth_remaining, bit_count, 0);
            if depth_remaining[depth - 1] == 0 { depth = depth_remaining.iter().enumerate().filter(|(_, x) | **x!= 0).max_by(|(i, _), (j, _)| i.cmp(j)).unwrap().0 + 1}
            if depth == 0 || depth_remaining.iter().sum::<u64>() == 0 { break; }
            println!("lit, depth {}, remaining {} ", depth, depth_remaining[depth - 1]);
            content = to_num(literal)
        }
        else {
            let mut bit_count = 7;
            depth += 1;
            depth_mode[depth - 1] = take_n(i, 1)[0] as i8;
            depth_remaining[depth - 1] = match depth_mode[depth - 1] {
                0 => {bit_count += 15; to_num(take_n(i, 15)) },
                _ => {bit_count += 11; to_num(take_n(i, 11)) }
            };
            println!("op, depth {}, remaining {} ", depth, depth_remaining[depth - 1]);
            reduce_remaining(depth, &depth_mode, &mut depth_remaining, bit_count, 1);
        }
    }
    version_sum
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

fn reduce_remaining(depth: usize, depth_mode: &Vec<i8>, depth_remaining: &mut Vec<u64>, by: u64, reduce: usize){
    for i in 0..depth - reduce {
        if depth_mode[i] == 0 { depth_remaining[i] -= by; }
    }
    if depth_mode[depth - 1] == 1 && depth_remaining[depth - 1] > 0 { depth_remaining[depth - 1] -= 1; }
}
