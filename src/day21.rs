use std::cmp::max;
use crate::helpers::{add_modular_no0, add_modular};
use cached::proc_macro::cached;
use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    play(parse_input(input), 1000)
}

pub fn part2(input: String) -> u64 {
    play_quantum(parse_input(input))
}

pub fn parse_input(input: String) -> Vec<u8> {
    input.lines().map(|l| l.parse::<u8>().unwrap()).collect::<Vec<_>>()
}

pub fn play(pos: Vec<u8>, win_score: u16) -> u64 {
    let mut die = 0;
    let mut pos = pos;
    let mut scores = vec![0; 2];
    let mut rolls = 0;
    let mut i = 0;
    loop {
        for _ in 0..3 {
            rolls += 1;
            die = add_modular_no0(die, 1, 100);;
            pos[i] = add_modular_no0(pos[i], die, 10);
        }
        scores[i] += pos[i] as u16;
        if scores[i] >= win_score { break; }
        i = !(i == 1) as usize;
    }
    *scores.iter().min().unwrap() as u64 * rolls
}

pub fn play_quantum(pos: Vec<u8>) -> u64 {
    let scores = play_round(pos[0], pos[1], 0, 0, 21);
    max(scores.0, scores.1)
}

#[cached]
pub fn play_round(pos1: u8, pos2: u8, score1: u8, score2: u8, win_score: u8) -> (u64, u64) {
    if score1 >= win_score { return (1, 0) }
    if score2 >= win_score { return (0, 1) }

    let mut score = (0, 0);

    for rolls in [1,2,3].iter().map(|_|[1,2,3].iter()).multi_cartesian_product() {
        let new_pos= add_modular_no0(pos1, rolls.into_iter().sum(), 10);
        let new_score = score1 + new_pos;
        let (p2_win,p1_win) = play_round(
                   pos2,
                   new_pos,
                   score2,
                   new_score, win_score);
        score = (score.0 + p1_win, score.1 + p2_win);
    };
    score
}