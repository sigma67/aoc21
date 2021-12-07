pub fn part1(input: String) -> u32 {
    find_optimal_position(&input, false)
}

pub fn part2(input: String) -> u32 {
    find_optimal_position(&input, true)
}

pub fn find_optimal_position(input: &String, increasing: bool) -> u32 {
    let positions: Vec<i16> = input.split(',')
        .map(|c| c.parse::<i16>().unwrap()).collect();
    let mut fuel_sums: Vec<u32> = Vec::new();
    let max_pos = *positions.iter().max().unwrap();
    for pos in 0..max_pos{
        let sum = positions.iter()
                .fold(0, | sum, p | {
                    let diff = (p - pos).abs() as u32;
                    sum + match increasing {
                        true => (diff * (diff + 1)) / 2,
                        false => diff
                }
            });
        fuel_sums.push(sum);
    }
    *fuel_sums.iter().min().unwrap()
}