#![allow(dead_code)]
#![feature(exclusive_range_pattern)]

use std::fs::File;
use std::io::Read;

use clap::*;

mod instruction;
mod value;
mod chunk;

fn main() -> Result<()> {
	let matches = clap_app!(app =>
		(name: "Lua Deserializer")
		(version: env!("CARGO_PKG_VERSION"))
		(author: &*env!("CARGO_PKG_AUTHORS").replace(":", ", "))
		(@arg INPUT: +required)
	).get_matches();

	let mut input = File::open(matches.value_of("INPUT").unwrap())?;
	let mut buffer = vec![0; input.metadata()?.len() as usize];

	input.read(&mut buffer)?;

	let chunk = chunk::parse(&buffer)
		.map_err(|error| clap::Error {
			message: error.to_string(),
			kind: ErrorKind::Io,
			info: None,
		})?.1;

	println!("{:#?}", chunk);

	Ok(())
}
