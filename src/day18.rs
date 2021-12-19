use std::cmp::max;
use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    add_snailfish(&mut parse_lines(input))
}

pub fn part2(input: String) -> u64 {
    let vals = parse_lines(input);
    let mut max_sum = 0;
    for window in vals.into_iter().combinations(2) {
        let mut sum = window.concat();
        for r in sum.iter_mut() {
            r.depth += 1;
        }
        reduce(&mut sum);
        max_sum = max(max_sum, magnitude(&mut sum));
    }
    max_sum
}

pub fn add_snailfish(vals: &mut Vec<Vec<SnailfishNum>>) -> u64 {
    let len = vals.len();
    for i in 1..len{
        let mut sum = vals[i-1..i+1].concat();
        for r in sum.iter_mut() {
            r.depth += 1;
        }
        reduce(&mut sum);
        vals[i] = sum;
    }
    magnitude(&mut vals[len-1])
}

fn parse_lines(input: String) -> Vec<Vec<SnailfishNum>> {
    let mut vals: Vec<Vec<SnailfishNum>> = Vec::new();
    for line in input.lines() {
        vals.push(parse_line(line));
    }
    vals
}

pub fn parse_line(line: &str) -> Vec<SnailfishNum> {
    let mut vals: Vec<SnailfishNum> = Vec::new();
    let mut depth = 0;
    let mut nums: Vec<u64> = Vec::new();
    for c in line.chars() {
        match c {
            '[' => {
                if !nums.is_empty() {
                    vals.push(sn(nums.pop().unwrap(), 0, depth, true));
                }
                depth += 1;
            },
            ']' => {
                match nums.len() {
                    1 => { vals.push(sn(nums[0], 0, depth, true)); nums.clear() },
                    2 => { vals.push(sn(nums[0], nums[1], depth, false)); nums.clear() },
                    _ => ()
                }
                depth -= 1;
            },
            ',' => continue,
            _ => nums.push(c as u64 - 0x30)
        }
    }
    vals
}

pub fn reduce(vals: &mut Vec<SnailfishNum>) {
    let mut action_taken = true;
    'main: loop {
        if !action_taken { break; }
        else { action_taken = false }
        for j in 0..vals.len() {
            if vals[j].depth == 5 {
                if j > 0 {
                    match vals[j - 1].is_single {
                        true => vals[j - 1].a += vals[j].a,
                        false => vals[j - 1].b += vals[j].a
                    }
                }
                if j < vals.len() - 1 {
                    vals[j + 1].a += vals[j].b;
                }
                move_up(j, |_, _| 0, vals);
                action_taken = true;
                continue 'main;
            }
        }
        for j in 0..vals.len() {
            if vals[j].is_single && vals[j].a >= 10 {
                let prior = vals[j].a;
                vals[j].a = prior / 2;
                vals[j].b = prior / 2 + prior % 2;
                vals[j].depth += 1;
                vals[j].is_single = false;
                action_taken = true;
            }
            else if vals[j].a >= 10 {
                let prior = vals[j].a;
                vals[j].a = vals[j].b;
                vals[j].b = 0;
                vals[j].is_single = true;
                vals.insert(j, sn(
                    prior / 2,
                    prior / 2 + prior % 2,
                    vals[j].depth + 1,
                    false
                ));
                action_taken = true;
            }
            else if vals[j].b >= 10 {
                let prior = vals[j].b;
                vals[j].b = 0;
                vals[j].is_single = true;
                vals.insert(j + 1, sn(
                    prior / 2,
                    prior / 2 + prior % 2,
                    vals[j].depth + 1,
                    false
                ));
                action_taken = true;
            }
            if action_taken { continue 'main; }
        }
    }
}

pub fn move_up(i: usize, new_value: fn(u64, u64) -> u64, vals: &mut Vec<SnailfishNum>){
    if i > 0 && vals[i - 1].is_single && vals[i - 1].depth == vals[i].depth - 1 {
        vals[i - 1].is_single = false;
        vals[i - 1].b = new_value(vals[i].a, vals[i].b);
        vals.remove(i);
    } else if i < vals.len() && vals[i + 1].is_single && vals[i + 1].depth == vals[i].depth - 1{
        vals[i + 1].is_single = false;
        vals[i + 1].b = vals[i + 1].a;
        vals[i + 1].a = new_value(vals[i].a, vals[i].b);
        vals.remove(i);
    }
    else {
        vals[i].is_single = true;
        vals[i].a = new_value(vals[i].a, vals[i].b);
        vals[i].b = 0;
        vals[i].depth -= 1;
    }
}

pub fn magnitude(vals: &mut Vec<SnailfishNum>) -> u64 {
    loop {
        for i in 0..vals.len() {
            let max_depth = vals.iter().map(|x| x.depth).max().unwrap();
            if !vals[i].is_single && vals[i].depth == max_depth {
                move_up(i, |a, b| 3 * a + 2 * b, vals);
                break;
            }
        }
        if vals.len() == 1 { break; }
    }
    3*vals[0].a + 2 * vals[0].b
}


#[derive(Debug, Clone)]
pub struct SnailfishNum {
    a: u64,
    b: u64,
    depth: u64,
    is_single: bool
}

fn sn(a: u64, b: u64, depth: u64, is_single: bool) -> SnailfishNum {
    SnailfishNum { a, b, depth, is_single }
}