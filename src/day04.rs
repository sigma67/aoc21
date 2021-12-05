pub fn part1(input: String) -> u32 {
    get_winner(input, true)
}

pub fn part2(input: String) -> u32 {
    get_winner(input, false)
}

type Board = [u8; 25];

pub fn get_winner(input: String, first: bool) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let draws: Vec<u8> = lines[0].split(',').map(|c| c.parse::<u8>().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut drawn: Vec<Board> = Vec::new();
    let mut wins: Vec<u8>;

    let mut board : Board = [0;25];
    for (i, line) in lines[2..lines.len()].iter().enumerate(){
        if i % 6 == 5 {
            boards.push(board.clone());
            board = [0;25];
            drawn.push(board.clone());
            continue;
        }
        let nums = line.split(' ').filter(|s| *s != "").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        for j in 0..5{
            board[(i % 6) * 5 + j] = nums[j];
        }
    }
    wins = vec![0; boards.len()];

    for draw in draws{
        for (b, board) in boards.iter().enumerate() {
            if wins[b] == 1 {
                continue;
            }
            let match_index = board.iter().position(|&r| r == draw);
            match match_index {
                Some(match_index) => drawn[b][match_index] = 1,
                None => continue
            }
            wins[b] = check_win(&drawn[b]);
            let win_condition = match first {
                true => wins[b] == 1,
                false => wins[b] == 1 && wins.iter().sum::<u8>() == boards.len() as u8
            };
            if win_condition {
                println!("{:?}", board);
                println!("{:?}", drawn[b]);
                return sum_unchecked(board, &drawn[b]) * draw as u32;
            }
        }
    }
    0
}


fn check_win(drawn: &[u8]) -> u8 {
    for i in 0..5 {     //rows
        let mut hsum: u8 = 0;
        let mut vsum: u8 = 0;
        for j in 0..5 { //cols
            hsum += drawn[i*5+j];
            vsum += drawn[j*5+i];
        }
        if (hsum == 5) || (vsum == 5){
            return 1
        }
    }
    0
}

fn sum_unchecked(board: &[u8], drawn: &[u8]) -> u32 {
    board.iter().enumerate().fold(0u32,
                                  | sum, (i, entry) |
                                      sum + ((1 - drawn[i]) * entry) as u32
    )
}