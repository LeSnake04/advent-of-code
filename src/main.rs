use crate::args::matches;
use anyhow::Result;
use dialoguer::Input;

pub use helper::*;

pub mod args;
pub mod helper;
pub mod y2022;

fn main() -> Result<()> {
	let args = matches();
	let day: u8 = match args.get_one("day") {
		Some(a) => *a,
		None => Input::<String>::new()
			.with_prompt("Enter the day")
			.interact_text()?
			.parse()?,
	};
	y2022::run(day)?;
	Ok(())
}
