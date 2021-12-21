use crate::helpers::to_num;
use std::collections::HashSet;

pub fn part1(input: String) -> u64 {
    enhance(input, 100, 2)
}

pub fn part2(input: String) -> u64 {
    enhance(input, 100, 50)
}

pub fn enhance(input: String, size: usize, steps: u8) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();

    let algo = lines[0].trim().chars().map(|c | pixel_to_int(c)).collect::<Vec<_>>();

    let mut lit_pixels: HashSet<usize> = HashSet::new();
    for (i, line) in lines[2..].iter().enumerate() {
        for (j, p) in line.chars().map(|c | pixel_to_int(c)).enumerate() {
            if p == 1 {
                lit_pixels.insert(i * size + j);
            }
        }
    }
    let mut size = size;
    let mut new_set= HashSet::new();
    let mut prev_set = lit_pixels;
    let mut num = vec![0_u8; 9];
    for step in 0..steps {
        let prev_size = size;
        size += 4;
        new_set = HashSet::with_capacity(size * size);
        for i in size..(size * size - size) {
            if i % size == 0 || i % size == (size - 1) { continue; }
            let indices = [i - size - 1, i - size, i - size + 1, i - 1, i, i + 1,
                i + size - 1, i + size, i + size + 1];
            for (j, k) in indices.iter().enumerate() {
                if *k < 2*size || k % size < 2 || k % size >= size - 2 || *k > size * size - 2*size {
                    num[j] = step % 2 * algo[0];
                }
                else if prev_set.contains(&((k - 2 * size) / size * prev_size + k % size - 2)) {
                    num[j] = 1;
                }
                else {
                    num[j] = 0;
                }
            }

            if algo[to_num(&num) as usize] == 1 {
                new_set.insert((i - size) / size * (size - 2) + i % size - 1);
            }
        }
        size -= 2;
        prev_set = new_set.clone();
    }
    new_set.len() as u64
}

fn pixel_to_int(c: char) -> u8 {
    match c {
        '#' => 1,
        '.' => 0,
        _ => panic!("wrong")
    }
}
