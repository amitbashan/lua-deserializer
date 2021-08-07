use nom::*;

named!(pub parse_a(&[u8]) -> u8, bits!(take_bits!(8usize)));

named!(pub parse_b(&[u8]) -> u8, bits!(take_bits!(8usize)));

named!(pub parse_c(&[u8]) -> u8, bits!(take_bits!(8usize)));

named!(pub parse_bx(&[u8]) -> u16, bits!(take_bits!(16usize)));
