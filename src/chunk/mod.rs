use nom::*;

use function::parse as parse_function;
use header::parse as parse_header;

use crate::value::parse_str;

pub mod header;
pub mod function;

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
		source_name: parse_str >>
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
