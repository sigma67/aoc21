use crate::helpers::to_num;

pub fn part1(input: String) -> u64 {
    let lines = parse_input(input);
    let gamma_seq = compute_gamma_seq(lines);
    let gamma = to_num(&gamma_seq);
    let delta = gamma_seq.iter().fold(
        0, |gamma, &bit| (gamma << 1) ^ !bit as u64
    );
    gamma * delta
}

pub fn part2(input: String) -> u64 {
    let lines = parse_input(input);

    get_support_rating(&lines, true) * get_support_rating(&lines, false)
}

fn parse_input(input: String) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(2).unwrap() as u8).collect()).collect()
}


fn compute_gamma_seq(lines: Vec<Vec<u8>>) -> Vec<u8>{
    let mut counts = vec![0; 12];
    let mut total: u16 = 0;
    for line in lines {
        for (i, c) in line.iter().enumerate() {
            counts[i] += *c as u16
        }
        total += 1;
    }
    counts.iter().map(|c| (*c > total / 2) as u8).collect::<Vec<_>>()
}

fn get_support_rating(lines: &Vec<Vec<u8>>, most_common: bool) -> u64 {
    let mut positions: [u8; 1000] = [1; 1000];
    for i in 0..lines[0].len() {
        let total = positions.iter().map(|&b| b as u16).sum::<u16>();
        if total == 1 { break; }
        let mut count: u16 = 0;

        //count ones in relevant positions
        for (j, line) in lines.iter().enumerate(){
            count += (*line.get(i).unwrap() & positions[j]) as u16;
        }

        //choose preferred depending on total count
        let selected: u8 = match most_common {
            true => (count as f32  >= total as f32 / 2.0) as u8,
            false => ((count as f32) < total as f32 / 2.0) as u8
        };

        // update relevant positions
        for (j, line) in lines.iter().enumerate() {
            positions[j] = positions[j] & (line[i] == selected) as u8;
        }
    }
    
    //compute rating
    let mut rating: u32 = 0;
    for (k, p) in positions.iter().enumerate(){
        if *p == 1 as u8 {
            rating = lines[k].iter().fold(
                0, |gamma, &bit| (gamma << 1) ^ bit as u32
            );
        }
    }
    rating as u64
}

