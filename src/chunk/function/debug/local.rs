use std::ops::Range;

use nom::{*, multi::*, number::complete::le_u32};

use crate::value::parse_str;

#[derive(Debug)]
pub struct Local<'a> {
	name: &'a str,
	range: Range<u32>,
}

impl Local<'_> {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Local>> {
		let (input, length) = le_u32(input)?;

		fn parse_single(input: &[u8]) -> IResult<&[u8], Local> {
			let (input, name) = parse_str(input)?;
			let (input, start) = le_u32(input)?;
			let (input, end) = le_u32(input)?;

			Ok((input,
				Local {
					name,
					range: (start..end),
				}
			))
		}

		count(parse_single, length as usize)(input)
	}
}
