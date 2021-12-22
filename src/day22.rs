use std::cmp::{min,max};
use itertools::{Combinations, Itertools};
use regex::Regex;

pub fn part1(input: String) -> u64 {
    let re = Regex::new(r"[-]?\d+").unwrap();
    let mut cuboids: Vec<Cuboid> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let n: Vec<_> = re.find_iter(line)
            .filter_map(|digits| digits.as_str().parse::<i64>().ok())
            .collect();
        let state = line.split_once(" ").unwrap().0 == "on";
        if n[0].abs() > 50 { continue; }
        cuboids.push(
            Cuboid::new(state, n[0],n[1],n[2], n[3], n[4], n[5], i)
        );
    }
    let len = cuboids.len();
    for i in 0..len {
        //println!("{} {}", i + 1, cuboids[i].volume_recursive());
        let b = &cuboids[i].clone();
        println!("CUBE {} {}", i, b.volume());
        for j in 0..i {
            println!("comp with cube {}", j+1);
            //let a = &cuboids[j].clone(); //prior ones
            //if !cuboids[j].is_on && !cuboids[i].is_on { continue }
            if !cuboids[j].is_on { continue; }
            let intersect = Cuboid::intersect(&cuboids[j], &b);
            if intersect.is_none() { continue; }
            let mut c = intersect.unwrap();
            c.step = i;
            if !b.is_on && cuboids[j].is_on { // turn off
                cuboids[j].sub(c);
            }
            else if b.is_on && cuboids[j].is_on { // relight
                cuboids[j].add(c);
            }
            // if b.is_on && !cuboids[j].is_on {
            //     cuboids[j].add(c);
            // }
            // else if cuboids[j].is_on {
            //     cuboids[j].sub(c)
            // }
        }
    }
    println!("total {:?}", cuboids.iter().map(|c| c.volume_recursive()).collect::<Vec<_>>());
    cuboids.iter().map(|c| c.volume_recursive()).sum()
}

pub fn part2(input: String) -> u64 {
    0
}

#[derive(Debug, Clone)]
pub struct Cuboid {
    is_on: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
    step: usize,
    adds: Vec<Cuboid>,
    subs: Vec<Cuboid>
}

impl Cuboid {
    pub fn new(state: bool,
               xmin: i64, xmax: i64, ymin: i64, ymax: i64, zmin: i64, zmax: i64,
               step: usize)  -> Cuboid {
        Cuboid {
            is_on: state,
            x: (xmin, xmax),
            y: (ymin, ymax),
            z: (zmin, zmax),
            step,
            adds: Vec::new(),
            subs: Vec::new()
        }
    }
    
    pub fn add(&mut self, c: Cuboid) {
        println!("add {}", &c.volume());
        self.adds.push(c)
    }

    pub fn sub(&mut self, c: Cuboid) {
        println!("sub {}", &c.volume());
        self.subs.push(c)
    }
    
    pub fn volume(&self) -> u64 {
        ((self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)) as u64
    }

    pub fn volume_recursive(&self) -> u64 {
        if !self.is_on { return 0 }
        let mut volume = self.volume();

        let mut relights = Vec::new();
        for a in self.adds.iter() {
            for b in self.subs.iter() {
                if a.step <= b.step { continue; }
                match Cuboid::intersect(a, b) {
                    Some(c) => relights.push(c),
                    _ => continue
                }
            }
        }
        println!("{:?}", self.adds);
        println!("{:?}", relights);
        volume += relights.iter().map(|c| c.volume()).sum::<u64>();

        let mut relight_intersections = Vec::new();
        for c in relights.iter().permutations(2) {
            //if c[1].step <= c[0].step { continue; }
            match Cuboid::intersect(c[0], c[1]) {
                Some(c) => relight_intersections.push(c),
                _ => continue
            }
        }
        volume -= relight_intersections.iter().map(|c| c.volume()).sum::<u64>();

        let mut intersections_off = Vec::new();
        for c in self.subs.iter().permutations(2) {
            match Cuboid::intersect(c[0], c[1]) {
                Some(c) => intersections_off.push(c),
                _ => continue
            }
        }
        println!("{:?}", self.subs);
        volume += intersections_off.iter().map(|c| c.volume()).sum::<u64>();
        volume -= self.subs.iter().map(|c| c.volume()).sum::<u64>();
        volume -= self.adds.iter().map(|c| c.volume()).sum::<u64>();

        // for sub in &self.subs {
        //     let sub_volume = sub.volume_recursive(false);
        //     if self.is_on == false && sub.is_on == true { volume += sub_volume }
        //     //else if self.is_on == false && sub.is_on == { false }
        //     else { volume -= sub_volume };
        // }
        volume
    }

    pub fn intersect(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
        //compute intersecting cuboid if any, else return
        let x = intersection_range((a.x.0, a.x.1), (b.x.0, b.x.1));
        let y = intersection_range((a.y.0, a.y.1), (b.y.0, b.y.1));
        let z = intersection_range((a.z.0, a.z.1), (b.z.0, b.z.1));
        if x.is_none() || y.is_none() || z.is_none() { return None }
        else {
            let c = Some(Cuboid::new(b.is_on, x.unwrap().0, x.unwrap().1,
                                    y.unwrap().0, y.unwrap().1, z.unwrap().0, z.unwrap().1, 0));
            //println!("{:?}", c.as_ref().unwrap());
            return c;
        };
    }

    pub fn segment(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
        let mut cuboids: Vec<Cuboid> = Vec::new();
        let xs = non_intersection_ranges((a.x.0, a.x.1), (b.x.0, b.x.1));
        let ys = non_intersection_ranges((a.y.0, a.y.1), (b.y.0, b.y.1));
        let zs = non_intersection_ranges((a.z.0, a.z.1), (b.z.0, b.z.1));
        println!("{:?} {:?} {:?}", xs, ys, zs);
        for x in xs {
            for y in &ys {
                for z in &zs {
                    cuboids.push(Cuboid::new(a.is_on, x.0, x.1, y.0, y.1, z.0, z.1, 0))
                }
            }
        }
        cuboids
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

pub fn non_intersection_ranges(a: (i64, i64), b: (i64, i64)) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    if b.0 < a.0 && b.1 > a.1 { ranges.push((a.0, a.1)) }
    if b.0 > a.0 { ranges.push((a.0, b.0)) }
    if b.1 < a.1 { ranges.push((b.1, a.1)) }
    ranges
}