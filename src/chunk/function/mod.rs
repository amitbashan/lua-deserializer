use nom::{*, number::complete::*};

use crate::{
	chunk::function::{
		debug::{
			local::{*, parse as parse_local},
			position::{*, parse as parse_positions},
			upvalue::parse as parse_upvalues,
		},
		vararg::{*, parse as parse_vararg},
	},
	instruction::{*, parse as parse_instruction},
	value::{*, parse as parse_value},
};

mod debug;
mod vararg;

#[derive(Debug)]
pub struct Function<'a> {
	pub line_defined: u32,
	pub last_line_defined: u32,
	pub vararg_flag: Flag,
	pub maximum_stack_size: u8,
	pub code: Vec<Instruction>,
	pub constants: Vec<Value<'a>>,
	pub closures: Vec<Function<'a>>,
	pub positions: Option<Vec<Position>>,
	pub locals: Option<Vec<Local<'a>>>,
	pub upvalues: Option<Vec<&'a str>>,
}

named!(
	pub parse(&[u8]) -> Function,
	do_parse!(
		line_defined: le_u32 >>
		last_line_defined: le_u32 >>
		upvalues_length: le_u8 >>
		parameters_length: le_u8 >>
		vararg_flag: parse_vararg >>
		maximum_stack_size: le_u8 >>
		code_length: le_u32 >>
		code: count!(parse_instruction, code_length as usize) >>
		constants_length: le_u32 >>
		constants: count!(parse_value, constants_length as usize) >>
		closures_length: le_u32 >>
		closures: count!(parse, closures_length as usize) >>
		positions: opt!(parse_positions) >>
		locals_length: le_u32 >>
		locals: opt!(count!(parse_local, locals_length as usize)) >>
		upvalues: opt!(parse_upvalues) >>
		(
			Function {
				line_defined,
				last_line_defined,
				vararg_flag,
				maximum_stack_size,
				code,
				constants,
				closures,
				positions,
				locals,
				upvalues,
			}
		)
	)
);
