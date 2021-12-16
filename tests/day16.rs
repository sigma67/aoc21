use advent_of_code::day16::{to_num, decode};

#[test]
fn test_day16_p1() {
    let input = String::from("8A004A801A8002F478");
    assert_eq!(decode(input), 16);
    let mut input = String::from("620080001611562C8802118E34");
    assert_eq!(decode(input), 12);
    input = String::from("C0015000016115A2E0802F182340");
    assert_eq!(decode(input), 23);
    input = String::from("A0016C880162017C3686B18A3D4780");
    assert_eq!(decode(input), 31);
}

#[test]
fn test_to_num() {
    let bits: Vec<u8> = vec![1,0,0];
    let result = to_num(bits);
    let expected: u64 = 4;
    assert_eq!(expected, result);
}
