use nom::{
	*,
	error::*,
	number::complete::*,
};

#[derive(Debug)]
pub enum Flag {
	Has,
	Is,
	Needs,
}

pub fn parse(input: &[u8]) -> IResult<&[u8], Flag> {
	let (input, flag) = le_u8(input)?;

	Ok((input,
		match flag {
			1 => Ok(Flag::Has),
			2 => Ok(Flag::Is),
			4 => Ok(Flag::Needs),
			_ => Err(
				Err::Failure(
					Error::from_error_kind(input, ErrorKind::Switch)
				)
			),
		}?
	))
}
