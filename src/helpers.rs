pub fn to_num(bits: &Vec<u8>) -> u64 {
    bits.iter().fold(
        0, |gamma, &bit| (gamma << 1) ^ bit as u64
    )
}
