use std::cmp::min;
use std::collections::HashMap;
use crate::adjacents::get_adjacent_indices;

pub fn part1(input: String) -> u64 {
    find_hotspots(input, 100)
}

pub fn part2(input: String) -> u64 {
    find_basins(input, 100)
}

pub fn parse_input(input: String) -> Vec<u8> {
    let mut map: Vec<u8> = Vec::new();
    for line in input.lines() {
        let nums: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        map.extend(nums)
    }
    map
}

pub fn find_hotspots(input: String, size:usize) -> u64 {
    let map = parse_input(input);
    let sum = map.iter().enumerate().fold(0, | sum: u64, (i, num) | {
        let indices = get_adjacent_indices(i, size, map.len(), false);
        for i in indices {
            if *num >= map[i] { return sum }
        }
        // if  (top(i, size) && (*num >= map[i - size]))   ||
        //     (left(i, size) && (*num >= map[i - 1]))     ||
        //     (right(i, size) && (*num >= map[i + 1]))    ||
        //     (bottom(i, size, map.len()) && (*num >= map[i + size]))
        //     { return sum }
        sum + (*num as u64) + 1
    });
    sum
}

pub fn find_basins(input: String, size:usize) -> u64 {
    let map = parse_input(input);
    let mut marker: u16 = 0;
    let mut basin_map: Vec<u16> = vec![0; map.len()];
    'outer: for (i, num) in map.iter().enumerate(){
        if *num != 9 {
            let indices = get_adjacent_indices(i, size, basin_map.len(), false);
            for j in indices{
                if basin_map[i] != 0 { basin_map[i] = basin_map[j]; continue 'outer; }
            }
            marker += 1;
            basin_map[i] = marker;
        }
    }

    let mut val = 1_u64;
    let mut prev = 0;
    while val > prev {
        for i in 0..basin_map.len() {
            let num = basin_map[i];
            if num != 0 {
                let indices = get_adjacent_indices(i, size, basin_map.len(), false);
                let filtered: Vec<&usize> = indices.iter().filter(|i| basin_map[**i] != 0).collect();
                let min = filtered.iter().fold(num, |marker, i| min(basin_map[**i], marker));
                for j in filtered { basin_map[*j] = min; }
                basin_map[i] = min;
            }
        }
        let mut map = HashMap::new();

        for c in basin_map.iter().filter(|x| **x != 0) {
            *map.entry(c).or_insert(0) += 1_u64;
        }
        let mut values = map.into_values().collect::<Vec<u64>>();
        values.sort_by(|a, b| b.cmp(a));

        prev = val;
        val = values[0] * values[1] * values[2]
    }
    val
}

