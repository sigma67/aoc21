use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    get_score(input, true)
}

pub fn part2(input: String) -> u64 {
    get_score(input, false)
}

pub fn get_score(input: String, errors: bool) -> u64{
    let scores_errors: HashMap<char, u64> = HashMap::from([
        (')', 3), (']', 57), ('}', 1197), ('>', 25137)
    ]);
    let scores_complete: HashMap<char, u64> = HashMap::from([
        (')', 1), (']', 2), ('}', 3), ('>', 4)
    ]);
    let mut sum = 0_u64;
    let brackets = ['(', '[', '{', '<', ')', ']', '}', '>'];
    let mut all_scores: Vec<u64> = Vec::new();
    for line in input.lines(){
        let mut brackets_line: Vec<usize> = Vec::new();
        let mut incorrect = false;
        for (i, c) in line.chars().enumerate(){
            let mut pos = brackets.iter().position(|&x| x == c).unwrap();
            if pos < 4 { brackets_line.push(pos); }
            else {
                let last_opening = brackets_line.pop().unwrap();
                if brackets[pos - 4] != brackets[last_opening] {
                    if errors { sum += scores_errors[&c]; }
                    incorrect = true;
                    break;
                }
            }
        }
        if !errors && !incorrect {
            let mut score = 0_u64;
            for i in 0..brackets_line.len(){
                let last_opening = brackets_line.pop().unwrap();
                score *= 5;
                score += scores_complete[&brackets[last_opening + 4]];
            }
            all_scores.push(score);
        }
    }
    if !errors{
        all_scores.sort();
        sum = all_scores[all_scores.len() / 2];
    }
    sum as u64
}
