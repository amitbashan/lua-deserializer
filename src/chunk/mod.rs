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

pub fn parse(input: &[u8]) -> IResult<&[u8], Chunk> {
	let (input, header) = header::parse(input)?;
	let (input, source_name) = parse_str(input)?;
	let (input, main) = function::Function::parse(input)?;

	Ok((input,
		Chunk {
			header,
			source_name,
			main,
		},
	))
}
