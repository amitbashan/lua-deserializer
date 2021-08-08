use nom::{*, number::complete::*};

#[derive(Debug)]
pub enum Flag {
	Has,
	Is,
	Needs,
}

named!(
	pub parse(&[u8]) -> Flag,
	switch!(
		le_u8,
		1 => value!(Flag::Has) |
		2 => value!(Flag::Is) |
		4 => value!(Flag::Needs)
	)
);
