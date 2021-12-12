use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    count_paths(input, false)
}

pub fn part2(input: String) -> u64 {
    count_paths(input,true)
}

// commented parts are for keeping track of all the paths
// only keeping track of count for part B to save memory
pub fn count_paths(input: String, twice: bool) -> u64 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines(){
        let caves = line.split('-').collect::<Vec<&str>>();
        let first = map.entry(caves[0]).or_insert(Vec::new());
        first.push(caves[1]);
        let second = map.entry(caves[1]).or_insert(Vec::new());
        second.push(caves[0]);
    }
    //let mut paths: Vec<Vec<&str>> = Vec::new();
    let mut path_count = 0;
    for cave in &map["start"] {
        let mut path = vec!["start", cave];
        //paths.extend(generate(&path, &map, twice));
        path_count += generate(&path, &map, twice)
    }
    //paths.len() as u64
    path_count
}

fn generate<'a>(path: &Vec<&'a str>, map: &'a HashMap<&str, Vec<&str>>, twice: bool)
                -> u64 //-> Vec<Vec<&'a str>>
{
    let mut new_path = path.to_vec();
    let last_entry = path.last().unwrap();
    //let mut paths: Vec<Vec<&str>> = Vec::new();
    let mut path_count = 0;

    for cave in &map[*last_entry] {
        let is_illegal = if twice { //part 2
            let mut map = HashMap::new();
            let lower_case = new_path.iter().filter(|cave| cave.chars().all(|c| c.is_ascii_lowercase()));
            let mut exists = false;
            for c in lower_case {
                *map.entry(c).or_insert(0) += 1_u64;
                if c == cave { exists = true;}
            }
            map.into_values().any(|v| v >= 2)  && exists
        } else {
            (cave.chars().all(|c| c.is_ascii_lowercase()) && new_path.contains(cave))
        };
        if is_illegal || *cave == "start" {
            continue;
        }
        new_path.push(cave);
        if *cave == "end" {
            //paths.push((new_path).to_vec());
            path_count += 1;
        } else {
            //paths.extend(generate(&new_path, map, twice));
            path_count += generate(&new_path, map, twice)
        }
        new_path.pop();
    }
    //paths
    path_count
}