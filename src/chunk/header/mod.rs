use nom::{*, number::complete::*};

mod version;
mod endianness;
mod integral_flag;

#[derive(Debug)]
pub struct Header {
    version_number: u8,
    version: version::Version,
    endianness: endianness::Endianness,
    size_of_integer: u8,
    size_of_size_t: u8,
    size_of_instruction: u8,
    size_of_lua_number: u8,
    integral_flag: integral_flag::IntegralFlag,
}

named!(
	pub parse(&[u8]) -> Header,
	do_parse!(
		tag!("\x1BLua") >>
		version_number: le_u8 >>
		version: switch!(
			le_u8,
			0 => value!(version::Version::Official)
		) >>
		endianness: switch!(
			le_u8,
			0 => value!(endianness::Endianness::Big) |
			1 => value!(endianness::Endianness::Little)
		) >>
		size_of_integer: le_u8 >>
		size_of_size_t: le_u8 >>
		size_of_instruction: le_u8 >>
		size_of_lua_number: le_u8 >>
		integral_flag: switch!(
			le_u8,
			0 => value!(integral_flag::IntegralFlag::FloatingPoint) |
			1 => value!(integral_flag::IntegralFlag::Integral)
		) >>
		(
			Header {
				version_number,
				version,
				endianness,
				size_of_integer,
				size_of_size_t,
				size_of_instruction,
				size_of_lua_number,
				integral_flag,
			}
		)
	)
);