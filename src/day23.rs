use regex::Regex;
use cached::proc_macro::cached;

pub fn part1(input: String) -> u64 {
    play_game(input, 2)
}

pub fn part2(mut input: String) -> u64 {
    input.insert_str(45, "  #D#C#B#A#  \r\n  #D#B#A#C#  \r\n");
    play_game(input, 4)
}

pub fn play_game(input: String, size: usize) -> u64 {
    let mut buckets = [[0; 4]; 4];
    let re = Regex::new(r"[A-Z]").unwrap();
    let input_indices: Vec<usize> = match size {
        2 => vec![2,3],
        _ => (2..6).collect()
    };
    let mut index = 0;
    for (i, line) in input.lines().enumerate() {
        if !input_indices.contains(&i) { continue };
        for (b, pod) in re.find_iter(line).map(|s| s.as_str().chars().next().unwrap() as usize - 64).enumerate() {
            buckets[b][index] = pod;
        }
        index += 1;
    }
    let hall = [0_usize; 11];
    move_pod(buckets, hall, 0, u64::MAX, size)
}

#[cached]
pub fn move_pod(buckets: [[usize;4];4], hall: [usize;11], cost: u64, min_cost: u64, size: usize) -> u64 {
    if cost > 60000  { return min_cost;}
    let mut min_cost = min_cost;
    let mut moves = Vec::new();
    let mut bucket_states: [usize;4] = [0; 4];
    for i in 0..bucket_states.len() {
        if bucket_states[i] == 2 { continue };
        let b = i+1;
        let bucket = &buckets[i][0..size];
        bucket_states[i] = bucket.iter().fold(0_usize, |acc, x|
            acc + (*x == b) as usize);
        let depth = bucket.iter().position(|x| *x != 0)
            .unwrap_or(bucket.len());
        bucket_states[i] = match bucket_states[i] {
            x if x == size => 2,
            x if depth + x == size => 10 + depth,
            _ => 0
        };
        if bucket_states[i] > 0 { continue }; // not moving out if ready
        moves.extend(get_hall_moves(bucket[depth], b*2, hall, 0, depth));
    }
    if bucket_states.iter().sum::<usize>() == 8 { return cost }
    //for each in hall, check bucket moves
    for i in 0..hall.len() {
        if hall[i] == 0 { continue }
        let state = bucket_states[hall[i] -1];
        if state > 10 {
            let start = if i > hall[i] * 2 { i - 1 } else { i + 1 };
            if range(start, hall[i] * 2).iter().fold(
                0, |acc, x| acc + hall[*x]) > 0 { continue; }
            moves.push(mv(hall[i], i, hall[i] * 2, 1, state - 10));
        }
    }

    for m in moves {
        let cost_per = 10_u64.pow((m.pod - 1) as u32);
        let b = match m.kind {
            0 => m.from,
            _ => m.to
        } / 2 - 1;
        let diff = (m.from as i8 - m.to as i8).abs() as u64 + 1;
        let extra_steps = m.depth - m.kind;
        let current_cost = cost + (extra_steps as u64 + diff) * cost_per;
        if current_cost >= min_cost { return min_cost }

        let mut buckets2 = buckets.clone();
        let mut hall2 = hall.clone();
        match m.kind {
            0 => {
                buckets2[b][m.depth] = 0;
                hall2[m.to] = m.pod;
            },
            _ => {
                buckets2[b][m.depth - 1] = m.pod;
                hall2[m.from] = 0;
            }
        }
        min_cost = min_cost.min(
            move_pod(buckets2, hall2, current_cost, min_cost, size)
        );
    }
    min_cost
}

struct Move {
    pod: usize,
    from: usize,
    to: usize,
    kind: usize,
    depth: usize
}

fn mv(pod: usize, from: usize, to: usize, kind: usize, depth: usize)
    -> Move { Move { pod, from, to, kind, depth } }

fn get_hall_moves(pod: usize, i: usize, hall: [usize;11], kind: usize, depth: usize)
    -> Vec<Move> {
    let mut moves = Vec::new();
    for j in range(i, 0){
        if j % 2 == 0 && (j >= 2 && j <= 8) { continue }
        if hall[j] != 0 { break };
        moves.push(mv(pod, i, j, kind, depth));
    }
    for j in range(i+1, hall.len()) {
        if j % 2 == 0 && j >= 2 && j <= 8 { continue }
        if hall[j] != 0 { break };
        moves.push(mv(pod, i, j, kind, depth));
    }
    moves
}

fn range(a: usize, b: usize) -> Vec<usize> {
    match a < b {
        true => (a..b).collect(),
        _ => (b..a).rev().collect()
    }
}