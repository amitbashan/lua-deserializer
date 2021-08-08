use nom::*;

use crate::instruction::argument::*;

pub mod argument;

#[derive(Debug)]
pub enum Instruction {
	ABC {
		operation_code: u8,
		a: u8,
		b: u8,
		c: u8,
	},
	ABx {
		operation_code: u8,
		a: u8,
		bx: u16,
	},
	AsBx {
		operation_code: u8,
		a: u8,
		sbx: i16,
	},
}

named!(parse_operation_code(&[u8]) -> u8, bits!(take_bits!(8usize)));

named!(
	pub parse_abc(&[u8]) -> (u8, u8, u8),
	do_parse!(
		a: parse_a >>
		b: parse_b >>
		c: parse_c >>
		((a, b, c))
	)
);

named!(
	pub parse_abx(&[u8]) -> (u8, u16),
	do_parse!(
		a: parse_a >>
		bx: parse_bx >>
		((a, bx))
	)
);

named!(
	pub parse(&[u8]) -> Instruction,
	do_parse!(
		operation_code: parse_operation_code >>
		instruction: switch!(
			value!(operation_code),
			0 |
			2..=4 |
			6 |
			8..=21 |
			23..=30 |
			33..=35 |
			37 => do_parse!(
				abc: parse_abc >>
				(
					call!(
						abc,
						|(a, b, c)| {
							Instruction::ABC {
								operation_code,
								a,
								b,
								c,
							}
						}
					))
				) |
			1 |
			5 |
			7 |
			36 => do_parse!(
				abx: parse_abx >>
				(
					call!(
						abx,
						|(a, bx)| {
							Instruction::ABx {
								operation_code,
								a,
								bx,
							}
						}
					))
				) |
			23 |
			31..=32 => do_parse!(
				bx: parse_abx >>
				(
					call!(
						bx,
						|(a, bx)| {
							Instruction::AsBx {
								operation_code,
								a,
								sbx: bx as i16,
							}
						}
					))
				)
		) >>
		(instruction)
	)
);
