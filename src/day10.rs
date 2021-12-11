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
    let mut sum = 0_u64;
    let brackets = ['(', '[', '{', '<', ')', ']', '}', '>'];
    let mut all_scores: Vec<u64> = Vec::new();
    'outer: for line in input.lines(){
        let mut brackets_line: Vec<usize> = Vec::new();
        for c in line.chars(){
            let pos = brackets.iter().position(|&x| x == c).unwrap();
            if pos < 4 { brackets_line.push(pos); }
            else {
                let last_opening = brackets_line.pop().unwrap();
                if brackets[pos - 4] != brackets[last_opening] {
                    if errors { sum += scores_errors[&c]; }
                    continue 'outer;
                }
            }
        }
        if !errors {
            let mut score = 0_u64;
            while let Some(element) = brackets_line.pop() {
                score *= 5;
                score += (element + 1) as u64;
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
