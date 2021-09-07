use std::ops::Range;

use nom::{*, multi::*, number::complete::le_u32};

use crate::value::parse_str;

#[derive(Debug)]
pub struct Local<'a> {
	name: &'a str,
	range: Range<u32>,
}

impl<'a> Local<'a> {
	pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Vec<Self>> {
		let (input, length) = le_u32(input)?;

		count(Self::parse_single, length as usize)(input)
	}

	fn parse_single(input: &'a [u8]) -> IResult<&'a [u8], Self> {
		let (input, name) = parse_str(input)?;
		let (input, start) = le_u32(input)?;
		let (input, end) = le_u32(input)?;

		Ok((input,
			Self {
				name,
				range: (start..end),
			}
		))
	}
}
