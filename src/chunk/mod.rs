use nom::{*, number::complete::*};

use function::parse as parse_function;
use header::parse as parse_header;

mod header;
mod function;

#[derive(Debug)]
pub struct Chunk<'a> {
	header: header::Header,
	source_name: &'a str,
	main: function::Function<'a>,
}

named!(
	pub parse(&[u8]) -> Chunk,
	do_parse!(
		header: parse_header >>
		source_name_length: le_u32 >>
		source_name: take_str!(source_name_length) >>
		main: parse_function >>
		(
			Chunk {
				header,
				source_name,
				main,
			}
		)
	)
);
