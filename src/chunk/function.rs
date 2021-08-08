use nom::{*, number::complete::*};

use crate::{
	instruction::{*, parse as parse_instruction},
	value::{*, parse as parse_value},
};

#[derive(Debug)]
pub struct Function<'a> {
	pub line_defined: u32,
	pub last_line_defined: u32,
	pub maximum_stack_size: u8,
	pub code: Vec<Instruction>,
	pub constants: Vec<Value<'a>>,
	pub closures: Vec<Function<'a>>,
}

named!(
	pub parse(&[u8]) -> Function,
	do_parse!(
		line_defined: le_u32 >>
		last_line_defined: le_u32 >>
		upvalues_length: le_u8 >>
		parameters_length: le_u8 >>
		is_vararg_flag: le_u8 >>
		maximum_stack_size: le_u8 >>
		code_length: le_u32 >>
		code: count!(parse_instruction, code_length as usize) >>
		constants_length: le_u32 >>
		constants: count!(parse_value, constants_length as usize) >>
		closures_length: le_u32 >>
		closures: count!(parse, closures_length as usize) >>
		(
			Function {
				line_defined,
				last_line_defined,
				maximum_stack_size,
				code,
				constants,
				closures,
			}
		)
	)
);
