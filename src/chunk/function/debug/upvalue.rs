use nom::{*, number::complete::*};
use nom::multi::count;

use crate::value::parse_str;

pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<&str>> {
	let (input, upvalues_length) = le_u32(input)?;
	let (input, upvalues) = count(parse_str, upvalues_length as usize)(input)?;

	Ok((input, upvalues))
}
