use std::ops::Range;

use nom::{*, number::complete::le_u32};

use crate::value::parse_str;

#[derive(Debug)]
pub struct Local<'a> {
	name: &'a str,
	range: Range<u32>,
}

named!(
	pub parse(&[u8]) -> Local,
	do_parse!(
		name: parse_str >>
		start: le_u32 >>
		end: le_u32 >>
		(
			Local {
				name,
				range: (start..end),
			}
		)
	)
);
