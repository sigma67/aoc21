use advent_of_code::day16::{to_num, decode};

const INPUT1: &str = "8A004A801A8002F478";
const INPUT2: &str = "620080001611562C8802118E34";
const INPUT3: &str = "C0015000016115A2E0802F182340";
const INPUT4: &str = "A0016C880162017C3686B18A3D4780";
const INPUT5: &str = "C200B40A82";
const INPUT6: &str = "04005AC33890";
const INPUT7: &str = "880086C3E88112";
const INPUT8: &str = "CE00C43D881120";
const INPUT9: &str = "D8005AC2A8F0";
const INPUT10: &str = "F600BC2D8F";
const INPUT11: &str = "9C005AC2F8F0";
const INPUT12: &str = "9C0141080250320F1802104A08";


#[test]
fn test_day16_p1() {
    assert_eq!(decode(INPUT1, true), 16);
    assert_eq!(decode(INPUT2, true), 12);
    assert_eq!(decode(INPUT3, true), 23);
    assert_eq!(decode(INPUT4, true), 31);
}

#[test]
fn test_day16_p2() {
    assert_eq!(decode(INPUT5, false), 3);
    assert_eq!(decode(INPUT6, false), 54);
    assert_eq!(decode(INPUT7, false), 7);
    assert_eq!(decode(INPUT8, false), 9);
    assert_eq!(decode(INPUT9, false), 1);
    assert_eq!(decode(INPUT10, false), 0);
    assert_eq!(decode(INPUT11, false), 0);
    assert_eq!(decode(INPUT12, false), 1);
}

#[test]
fn test_to_num() {
    let bits: Vec<u8> = vec![1,0,0];
    let result = to_num(bits);
    let expected: u64 = 4;
    assert_eq!(expected, result);
}
