use nom::*;

use crate::value::parse_str;

pub mod header;
pub mod function;

#[derive(Debug)]
pub struct Chunk<'a> {
	header: header::Header,
	source_name: &'a str,
	main: function::Function<'a>,
}

impl<'a> Chunk<'a> {
	pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
		let (input, header) = header::Header::parse(input)?;
		let (input, source_name) = parse_str(input)?;
		let (input, main) = function::Function::parse(input)?;

		Ok((input,
			Self {
				header,
				source_name,
				main,
			},
		))
	}
}
