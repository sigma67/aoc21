use crate::helpers::to_num;
use std::collections::HashSet;
use crate::adjacents::get_adjacent_indices;

pub fn part1(input: String) -> u64 {
    enhance(input, 100)
}

pub fn part2(input: String) -> u64 {
    enhance(input, 100)
}

pub fn enhance(input: String, size: usize) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();

    let algo = lines[0].chars().map(|c | pixel_to_int(c)).collect::<Vec<_>>();

    let mut lit_pixels: HashSet<usize> = HashSet::new();
    for (i, line) in lines[2..].iter().enumerate() {
        for (j, p) in line.chars().map(|c | pixel_to_int(c)).enumerate() {
            if p == 1 {
                lit_pixels.insert(i * size + j);
            }
        }
    }
    //println!("{:?}", lit_pixels);
    let mut size = size;

    let mut prev_set = lit_pixels;
    let mut new_set = HashSet::new();
    for _ in 0..2 {
        let prev_size = size;
        size += 4;
        new_set = HashSet::new();
        for i in (size + 1)..(size * size - size) {
            if i % size == 0 || i % size == (size - 1) { continue; }
            let mut indices = get_adjacent_indices(i, size, size * size, true);
            indices.push(i);
            indices.sort();
            let mut num = vec![0_u8; 9];
            for (j, k) in indices.iter().enumerate() {
                if *k < 2*size || k % size < 2 || k % size > (size - 3) || *k > (size * size - 2*size) { continue; }
                //println!("{} {}", k, (k - 2*size) / size * prev_size + k % size - 2);
                if prev_set.contains(&((k - 2*size) / size * prev_size + k % size - 2)) {
                    num[j] = 1;
                }
            }
            if algo[to_num(&num) as usize] == 1 {
                new_set.insert(i);
            }
        }
        for j in 0..size * size{
            if new_set.contains(&j){  print!("#"); }
            else { print!(".") }
            if j % size == size - 1 { print!("\n"); }
        }
        //println!("{:?}", new_set);
        prev_set = new_set.clone();
    }
    new_set.len() as u64
}

fn pixel_to_int(c: char) -> u8 {
    match c {
        '#' => 1,
        _ => 0
    }
}
