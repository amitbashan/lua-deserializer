use nom::{*, multi::*, number::complete::*};

#[derive(Debug)]
pub struct Position {
	pub instruction: u32,
	pub source: u32,
}

impl Position {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Position>> {
		let (input, positions_length) = le_u32(input)?;
		let (input, source_positions) = count(le_u32, positions_length as usize)(input)?;

		Ok((input,
			(0..positions_length)
				.zip(source_positions)
				.map(
					|(instruction, source)|
						Position {
							instruction,
							source,
						}
				).collect()
		))
	}
}
