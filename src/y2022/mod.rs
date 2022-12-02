use anyhow::{anyhow, Result};

pub mod d01;
pub mod d02;

pub fn run(day: u8) -> Result<()> {
	match day {
		1 => d01::main()?,
		2 => d02::main(),
		d => Err(anyhow!("Invalid day: {}", d))?,
	};
	Ok(())
}
