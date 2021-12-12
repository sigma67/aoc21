use std::collections::HashMap;
extern crate rayon;
use rayon::prelude::*;

pub fn part1(input: String) -> u64 {
    count_paths(input, false)
}

pub fn part2(input: String) -> u64 {
    count_paths(input,true)
}

pub fn count_paths(input: String, p2: bool) -> u64 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines(){
        let caves = line.split('-').collect::<Vec<&str>>();
        let first = map.entry(caves[0]).or_insert(Vec::new());
        if caves[1] != "start" { first.push(caves[1]); }
        let second = map.entry(caves[1]).or_insert(Vec::new());
        if caves[0] != "start" { second.push(caves[0]); }
    }
    //let mut paths: Vec<Vec<&str>> = Vec::new();
    let mut path_count =
    &map["start"].par_iter().fold(|| 0_u64, |a: u64, x: &&str| {
        let mut path = vec!["start", x];
        a + generate(&path, &map, p2)
    }).sum::<u64>();
    *path_count
}

fn generate<'a>(path: &Vec<&'a str>, map: &'a HashMap<&str, Vec<&str>>, p2: bool) -> u64
{
    let last_entry = path.last().unwrap();

    map[*last_entry].par_iter().fold(|| 0_u64, |a: u64, cave: &&str| {
        let mut new_path = path.to_vec();
        let mut path_count = 0_u64;
        let is_illegal = if p2 { //part 2
            check_occurrences(cave, &new_path)
        } else {
            (is_lowercase(cave) && new_path.contains(cave))
        };
        if is_illegal || *cave == "start" { return 0; }
        new_path.push(cave);
        if *cave == "end" { path_count += 1; }
        else { path_count += generate(&new_path, map, p2) }
        path_count
    }).sum::<u64>()
}

fn check_occurrences(cave: &str, new_path: &Vec<&str>) -> bool {
    if !is_lowercase(cave) || cave == "end" { return false }
    let mut map = HashMap::new();
    let lower_case = new_path.iter().filter(|cave| is_lowercase(cave));
    let mut exists = false;
    for c in lower_case {
        *map.entry(c).or_insert(0) += 1_u64;
        if *c == cave { exists = true;}
    }
    map.into_values().any(|v| v >= 2)  && exists
}

fn is_lowercase(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}