use std::collections::HashSet;

pub fn part1(input: String) -> u64 {
    fold(input, 1, false)
}

pub fn part2(input: String) -> u64 {
    fold(input, 12, true)
}

pub fn fold(input: String, fold_count: usize, print: bool) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let split_index = lines.iter().position(|p| *p == "").unwrap();
    let mut points:Vec<(u16, u16)> = Vec::new();
    for i in 0..split_index {
        let (x, y) = lines[i].split_once(",").unwrap();
        points.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    let mut folds:Vec<(bool, u16)> = Vec::new();
    for i in split_index+1..lines.len(){
        let line = lines[i].replace("fold along ", "");
        let (t, x) = line.split_once("=").unwrap();
        folds.push((t == "x", x.parse().unwrap()));
    }
    for f in 0..fold_count {
        for i in 0..points.len(){
            points[i] = match folds[f].0 {
                true => (fold_point(points[i].0, folds[f].1), points[i].1),
                false => (points[i].0, fold_point(points[i].1, folds[f].1))
            };
        }
    }
    if print {
        let maxx = points.iter().map(|(v, _)| *v).max().unwrap();
        let maxy = points.iter().map(|(_, v)| *v).max().unwrap();
        for y in 0..maxy+1 {
            for x in 0..maxx+1 {
                let mut char = ".";
                let py = points.iter().fold(0, |acc, v| acc + (v.0 == x && v.1 == y) as u8);
                if py > 0 {
                    char = "#";
                }
                print!("{}", char);
            }
            print!("\n");
        }
    }
    let set = HashSet::<(u16, u16)>::from_iter(points);
    set.len() as u64
}

fn fold_point(p: u16, fold: u16) -> u16 {
    fold - (fold as i16 - p as i16).abs() as u16
}
