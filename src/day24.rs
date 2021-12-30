use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    parse(input, false)
}

pub fn part2(input: String) -> u64 {
    parse(input, true)
}

pub fn parse(input: String, smallest: bool) -> u64 {
    let mut instructions: [[i32;3];14] = [[0;3]; 14];
    for (i, line) in input.lines().enumerate() {
        let idx = match i % 18 {
            4 => 0,
            5 => 1,
            15 => 2,
            _ => continue
        };
        let c = line.split(" ").last().unwrap().parse::<i32>().unwrap();
        instructions[i / 18][idx] = c;
    }
    
    let mut inputs: HashMap<i32, u64> = HashMap::new();// = [HashMap::new(); 14];
    for (i, c) in instructions.into_iter().enumerate() {
        if i == 0 {
            for w in 1..10_u64 {
                inputs.entry(alu(c, w as i32, 0)).or_insert(
                    w * 10_u64.pow(13));
            }
        }
        else {
            let mut outputs: HashMap<i32, u64> = HashMap::with_capacity(inputs.len());
            for w in 1..10_u64 {
                for (k, old_w) in &inputs {
                    let new_z = alu(c, w as i32, *k);
                    let new_w = old_w + w * 10_u64.pow(13 - i as u32);
                    let key = outputs.entry(new_z).or_insert(new_w);
                    if smallest && new_w < *key || !smallest && new_w > *key {
                        *key = new_w
                    }
                }
            }
            inputs = outputs;
        }
    }
    *inputs.get(&0).unwrap()
}

fn alu(c: [i32;3], w: i32, mut z: i32) -> i32 {
    let (mut x, mut y);
    x = 1 - ((z % 26 + c[1]) == w) as i32;
    z /= c[0];
    y = 25 * x + 1;
    z *= y;
    z += (w + c[2]) * x;
    z
}