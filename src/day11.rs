use crate::adjacents::get_adjacent_indices;
use crate::day09::parse_input;

pub fn part1(input: String) -> u64 {
    count_flashes(input, 100)
}

pub fn part2(input: String) -> u64 {
    count_flashes(input, 300)
}

pub fn count_flashes(input: String, rounds: usize) -> u64 {
    let mut matrix = parse_input(input);
    let length = matrix.len() as usize;
    let mut flash_count = 0_u64;
    for r in 0..rounds {
        for i in 0..length {
            matrix[i] += 1;
        }
        for i in 0..length {
            if matrix[i] >= 10 {
                let flashes = flash(i, &mut matrix);
                if flashes == length as u64 { return r as u64 + 1; }
                flash_count += flashes;
            }
        }
        println!("Round {}, {} flashes", r + 1, flash_count);
    }
    flash_count
}

fn flash(index: usize, matrix: &mut Vec<u8>) -> u64{
    let indices = get_adjacent_indices(index, 10, matrix.len(), true);
    matrix[index] = 0;
    let mut count = 1;
    for j in indices {
        let k = j as usize;
        match matrix[k] {
            0 => { continue; },
            d if d >= 9 => { count += flash(k, matrix); },
            _ => { matrix[k] += 1; }
        }
    }
    count
}