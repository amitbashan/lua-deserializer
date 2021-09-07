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

impl Flag {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, flag) = le_u8(input)?;

		Ok((input,
			match flag {
				1 => Ok(Self::Has),
				2 => Ok(Self::Is),
				4 => Ok(Self::Needs),
				_ => Err(
					Err::Failure(
						Error::from_error_kind(input, ErrorKind::Switch)
					)
				),
			}?
		))
	}
}
