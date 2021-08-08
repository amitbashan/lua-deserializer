use nom::{*, number::complete::*};

use crate::value::parse_str;

named!(
	pub parse(&[u8]) -> Vec<&str>,
	do_parse!(
		upvalues_length: le_u32 >>
		upvalues: count!(parse_str, upvalues_length as usize) >>
		(upvalues)
	)
);
