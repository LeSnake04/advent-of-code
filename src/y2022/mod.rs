use anyhow::{anyhow, Result};

pub mod d01;

pub fn run(day: u8) -> Result<()> {
	match day {
		1 => d01::main(),
		d => Err(anyhow!("Invalid day: {}", d))?,
	}?;
	Ok(())
}
