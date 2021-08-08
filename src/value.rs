use nom::{*, number::complete::*};

#[derive(Debug)]
pub enum Value<'a> {
	Nil,
	Boolean(bool),
	Number(f64),
	String(&'a str),
}

named!(
	pub parse(&[u8]) -> Value,
	do_parse!(
		value: switch!(
			le_u8,
			0 => value!(Value::Nil) |
			1 => do_parse!(
				value: le_u8 >>
				(Value::Boolean(value != 0))
			) |
			3 => do_parse!(
				value: le_f64 >>
				(Value::Number(value))
			) |
			4 => do_parse!(
				length: le_u32 >>
				string: take_str!(length) >>
				(Value::String(string))
			)
		) >>
		(value)
	)
);
