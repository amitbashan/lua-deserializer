use nom::{
	*,
	error::*,
	number::complete::{le_u16, le_u8},
};

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

impl Instruction {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Instruction> {
		let (input, operation_code) = Self::parse_operation_code(input)?;

		match operation_code {
			0 |
			2..=4 |
			6 |
			8..=21 |
			24..=30 |
			33..=35 |
			37 => {
				let (input, abc) = Self::parse_abc(input)?;

				Ok((input, Instruction::from((operation_code, abc))))
			}
			1 |
			5 |
			7 |
			36 => {
				let (input, abx) = Self::parse_abx(input)?;

				Ok((input, Instruction::from((operation_code, abx))))
			}
			23 |
			31..=32 => {
				let (input, (a, bx)) = Self::parse_abx(input)?;

				Ok((input, Instruction::from((operation_code, (a, bx as i16)))))
			}
			_ => Err(
				Err::Failure(
					Error::from_error_kind(input, ErrorKind::Switch)
				)
			),
		}
	}

	fn parse_operation_code(input: &[u8]) -> IResult<&[u8], u8> {
		le_u8(input)
	}

	fn parse_abc(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
		let (input, a) = le_u8(input)?;
		let (input, b) = le_u8(input)?;
		let (input, c) = le_u8(input)?;

		Ok((input, (a, b, c)))
	}

	fn parse_abx(input: &[u8]) -> IResult<&[u8], (u8, u16)> {
		let (input, a) = le_u8(input)?;
		let (input, bx) = le_u16(input)?;

		Ok((input, (a, bx)))
	}
}

impl From<(u8, (u8, u8, u8))> for Instruction {
	fn from((operation_code, (a, b, c)): (u8, (u8, u8, u8))) -> Self {
		Self::ABC {
			operation_code,
			a,
			b,
			c,
		}
	}
}

impl From<(u8, (u8, u16))> for Instruction {
	fn from((operation_code, (a, bx)): (u8, (u8, u16))) -> Self {
		Self::ABx {
			operation_code,
			a,
			bx,
		}
	}
}

impl From<(u8, (u8, i16))> for Instruction {
	fn from((operation_code, (a, sbx)): (u8, (u8, i16))) -> Self {
		Self::AsBx {
			operation_code,
			a,
			sbx,
		}
	}
}
