use std::cmp::max;

pub fn part1(input: String) -> u64 {
    shoot(&input, false)
}

pub fn part2(input: String) -> u64 {
    shoot(&input, true)
}

pub fn shoot(input: &str, count: bool) -> u64{
    let split = input.trim().split_once(", ").unwrap();
    let (txmin, txmax) = split_coords(split.0);
    let (tymin, tymax) = split_coords(split.1);
    let (sx, sy) = (0, 0);
    let vxmin = find_vxmin(txmin);
    let vxmax = txmax;
    let mut ymax = 0;
    let mut tcount = 0;
    for vx in vxmin..vxmax + 1 {
        'main: for vy in -txmin/2..txmin/2 {
            let mut px = sx;
            let mut py = sy;
            let (mut vxc, mut vyc) = (vx, vy);
            let mut is_valid = true;
            let mut ymaxc = 0;
            while is_valid {
                px += vxc;
                py += vyc;
                ymaxc = max(ymaxc, py);
                vxc = match vxc {
                    x if x < 0 => vxc + 1 ,
                    x if x == 0 => vxc,
                    _ => vxc - 1
                };
                vyc -= 1;
                if (px >= txmin && px <= txmax) &&
                    (py >= tymin && py <= tymax) {
                    ymax = max(ymaxc, ymax);
                    tcount += 1;
                    continue 'main;
                } else {
                    is_valid = py >= tymin && px <= txmax;
                }
            }
        }
    }
    if count { tcount } else { ymax as u64 }
}

fn split_coords(input: &str) -> (i32, i32){
    input.split_once("=").unwrap().1
        .split_once("..").map(|(a,b)|
        (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unwrap()
}

fn find_vxmin(txmin: i32) -> i32 {
    let mut sum = 0;
    for i in 0..txmin {
        sum += i;
        if sum >= txmin {
            return i;
        }
    }
    0
}
