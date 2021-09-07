use nom::{
	*,
	bytes::complete::take,
	error::*,
	number::complete::*,
};

#[derive(Debug)]
pub enum Value<'a> {
	Nil,
	Boolean(bool),
	Number(f64),
	String(&'a str),
}

impl<'a> Value<'a> {
	pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
		let (input, kind) = le_u8(input)?;

		match kind {
			0 => Ok((input, Self::Nil)),
			1 => {
				let (input, value) = le_u8(input)?;

				Ok((input, Self::Boolean(value != 0)))
			}
			3 => {
				let (input, value) = le_f64(input)?;

				Ok((input, Self::Number(value)))
			}
			4 => {
				let (input, value) = parse_str(input)?;

				Ok((input, Self::String(value)))
			}
			_ => Err(
				Err::Failure(
					Error::from_error_kind(input, ErrorKind::Switch)
				)
			),
		}
	}
}

pub fn parse_str(input: &[u8]) -> IResult<&[u8], &str> {
	let (input, name_length) = le_u32(input)?;
	let (input, string) = take(name_length as usize)(input)?;

	Ok((input,
		std::str::from_utf8(string)
			.map_err(|_| nom::Err::Failure(Error::from_error_kind(input, ErrorKind::Fail)))?
	))
}
