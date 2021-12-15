use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::adjacents::get_adjacent_indices;
use crate::day09::parse_input;
use crate::day06::add_modular;

pub fn part1(input: String) -> u64 {
    let map = parse_input(input);
    find_min_risk(100, &map)
}

pub fn part2(input: String) -> u64 {
    let map = parse_input(input);
    let size = 100;
    //increase map 25x
    let map5 = map5(size, map);
    find_min_risk(size*5, &map5)
}

pub fn find_min_risk(size: usize, map: &Vec<u8>) -> u64 {
    let mut costs: Vec<u64> = vec![u64::MAX; map.len()];
    let mut heap = BinaryHeap::new();
    let vtx = 0;
    costs[vtx] = 0;
    heap.push(Reverse(vtx));
    while heap.len() > 0 { //dijkstra with priority queue
        let vtx = heap.pop().unwrap().0;
        let indices = get_adjacent_indices(vtx, size, map.len(), false);
        for i in indices{
            let alt = costs[vtx] + map[i] as u64;
            if alt < costs[i] {
                costs[i] = alt;
                heap.push(Reverse(i));
            }
        }
    }
    costs[costs.len() - 1]
}

pub fn map5(size: usize, map: Vec<u8>) -> Vec<u8>{
    let size5 = size*5;
    let mut map5 = vec![0_u8; map.len()*25];
    for i in 0..(size * size5){
        let j = i % size;
        let line = i / size5;
        map5[i] = add_modular_no0(map[line*size + j], (i / size % 5) as u8, 10);
    }
    for i in (size * size5)..map5.len(){
        let line = i % (size5*size) / size5;
        map5[i] = add_modular_no0(map5[line*size5 + i % size5], (i / (size5*size)) as u8, 10);
    }
    //println!("{:?}", &map5[0..10000]);
    map5
}

fn add_modular_no0(val: u8, add: u8, base: u8) -> u8 {
    if (add + val ) >= base { ((add + val) % base) + 1 }
    else { add + val }
}
