use nom::{*, number::complete::*};

#[derive(Debug)]
pub struct Position {
	pub instruction: u32,
	pub source: u32,
}

named!(
	pub parse(&[u8]) -> Vec<Position>,
	do_parse!(
		positions_length: le_u32 >>
		sources: count!(le_u32, positions_length as usize) >>
		positions:
			value!((0..positions_length)
				.zip(sources)
				.map(
					|(instruction, source)|
						Position {
							instruction,
							source,
						}
			).collect()) >>
		(positions)
	)
);
