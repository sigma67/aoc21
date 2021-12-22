use std::cmp::{min,max};
use itertools::{Combinations, Itertools};
use regex::Regex;

pub fn part1(input: String) -> u64 {
    count_cubes(input, true)
}

pub fn part2(input: String) -> u64 {
    count_cubes(input, false)
}

pub fn count_cubes(input: String, restrict: bool) -> u64 {
    let re = Regex::new(r"[-]?\d+").unwrap();
    let mut cuboids: Vec<Cuboid> = Vec::new();
    for line in input.lines() {
        let n: Vec<_> = re.find_iter(line)
            .filter_map(|digits| digits.as_str().parse::<i64>().ok())
            .collect();
        let state = (line.split_once(" ").unwrap().0 == "on") as i64;
        if restrict && n[0].abs() > 50 { continue; }
        cuboids.push(
            Cuboid::new(state, n[0],n[1],n[2], n[3], n[4], n[5])
        );
    }
    let len = cuboids.len();
    let mut volumes: Vec<Cuboid> = Vec::new();
    for i in 0..len {
        let b = cuboids[i].clone();
        let mut new_volumes: Vec<Cuboid> = Vec::new();
        for v in &volumes {
            match Cuboid::intersect(&b, v) {
                Some(mut c) => {
                    c.state = -1 * c.state;
                    new_volumes.push(c);
                }
                None => continue
            }
        }
        volumes.push(b);
        volumes.extend(new_volumes);
    }
    volumes.iter().map(|c| c.volume()).sum::<i64>() as u64
}

#[derive(Debug, Clone)]
pub struct Cuboid {
    state: i64,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64)
}

impl Cuboid {
    pub fn new(state: i64,
               xmin: i64, xmax: i64, ymin: i64, ymax: i64, zmin: i64, zmax: i64) -> Cuboid {
        Cuboid {
            state,
            x: (xmin, xmax),
            y: (ymin, ymax),
            z: (zmin, zmax)
        }
    }
    
    pub fn volume(&self) -> i64 {
        ((self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)) * self.state
    }

    pub fn intersect(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
        //compute intersecting cuboid if any, else return
        let x = intersection_range((a.x.0, a.x.1), (b.x.0, b.x.1));
        let y = intersection_range((a.y.0, a.y.1), (b.y.0, b.y.1));
        let z = intersection_range((a.z.0, a.z.1), (b.z.0, b.z.1));
        if x.is_none() || y.is_none() || z.is_none() { return None }
        else {
            let c = Some(Cuboid::new(b.state, x.unwrap().0, x.unwrap().1,
                                     y.unwrap().0, y.unwrap().1, z.unwrap().0, z.unwrap().1));
            return c;
        };
    }
}

pub fn intersection_range(a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    if a.0 > b.1 || b.0 > a.1 { return None }
    let mut range = Some(match a.0 {
        x if x <= b.0 => (b.0, min(a.1, b.1)),
        _ => (a.0, min(a.1, b.1))
    });
    range
}