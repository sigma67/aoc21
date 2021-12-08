use std::collections::HashMap;

pub fn part1(input: String) -> u32 {
    count_unique(input)
}

pub fn part2(input: String) -> u32 {
    decode_input(input)
}

pub fn count_unique(input: String) -> u32 {
    let unique: [u8; 4] = [2,3,4,7];
    let mut count = 0;
    for line in input.lines(){
        let output = line.split(" | ").collect::<Vec<&str>>()[1];
        println!("{}", output);
        count += output.split(" ")
            .fold(0, |sum, x| sum + unique.contains(&(x.len() as u8)) as u32 );
    }
    count
}

pub fn decode_input(input: String) -> u32 {
    let mut mapping: [i8; 7];
    let mut sum = 0;
    for line in input.lines() {
        mapping = [-1; 7];
        let split = line.split(" | ").collect::<Vec<&str>>();
        let input = split[0].split(" ").collect::<Vec<&str>>();
        let output = split[1].split(" ").collect::<Vec<&str>>();
        let four = input.iter().filter(|x| x.len() == 4).collect::<Vec<&&str>>()[0];
        let mut map = HashMap::new();

        for c in split[0].chars().filter(|x| *x != ' ') {
            *map.entry(c).or_insert(0) += 1;
        }
        for i in 0..mapping.len() {
            if mapping[i] != -1 {continue;}
            let c = (i as u8 + 97) as char;
            let c_count = map.get(&c).unwrap();
            mapping[i] = match c_count {
                4 => 4,
                6 => 1,
                7 => { if (**four).contains(c) { 3 } else { 6 }}
                8 => { if (**four).contains(c) { 2 } else { 0 }}
                9 => 5,
                _ => panic!("odd input")
            }
        }
        let mut number = 0;
        for (i, s) in output.iter().enumerate(){
            number += 10_u32.pow(3-(i as u32))*string_to_number(s, mapping);
        }
        sum += number;
    }
    sum
}

fn string_to_number(input: &str, mapping: [i8; 7]) -> u32 {
    let mut chars: Vec<char> = Vec::new();
    for c in input.chars(){
        let i = mapping[((c as u8) - 97) as usize];
        chars.push((i as u8 + 97) as char);
    }
    chars.sort();
    let decoded: &str = &chars.iter().collect::<String>();
    match decoded {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => 0
    }
}
