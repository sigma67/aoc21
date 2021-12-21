pub fn to_num(bits: &Vec<u8>) -> u64 {
    bits.iter().fold(
        0, |gamma, &bit| (gamma << 1) ^ bit as u64
    )
}

pub fn add_modular_no0(val: u8, add: u8, base: u8) -> u8 {
    let m = (add + val) % base;
    match m {
        0 => base,
        _ => m
    }
}

pub fn sub_modular(val: u8, sub: u8, base: u8) -> usize {
    let val = val as i8;
    let sub = sub as i8;
    let base = base as i8;
    let diff = val % base - sub % base;
    match diff >= 0 {
        true => (val - sub) as usize,
        false => (diff + base) as usize
    }
}

pub fn add_modular(val: u8, add: u8, base: u8) -> u8 {
    (add + val) % base
}
