use std::cmp::min;
use std::cmp::max;

pub fn part1(input: String) -> u64 {
    find_overlaps(input, 1000, false)
}

pub fn part2(input: String) -> u64 {
    find_overlaps(input, 1000, true)
}

pub fn find_overlaps(input: String, size:usize, diagonals: bool) -> u64{
    let mut map: Vec<u8> = vec![0; size * size];
    for line in input.lines(){
        let vent: Vec<usize> = line.replace(" -> ", ",").split(",")
            .map(|c| c.parse::<usize>().unwrap()).collect();
        let is_diagonal = (vent[0] != vent[2]) && (vent[1] != vent[3]);
        if !diagonals && is_diagonal {
            continue;
        }

        let minx= min(vent[0], vent[2]);
        let maxx = max(vent[0],vent[2]);
        let miny = min(vent[1], vent[3]);
        let maxy = max(vent[1],vent[3]);
        let is_down = (vent[0] <= vent[2]) && (vent[1] <= vent[3]) ||
            (vent[0] >= vent[2]) && (vent[1] >= vent[3]);

        for x in minx..maxx + 1{
            if is_diagonal{
                match is_down {
                    true => map[(miny + (x-minx))* size + x] += 1,
                    false => map[(maxy - (x-minx))* size + x] += 1
                }
            }
            else {
                for y in miny..maxy + 1 {
                    map[y * size + x] += 1
                }
            }
        }
    }
    map.iter().fold(0u64, |sum, val| sum + (*val >= 2) as u64)
}
