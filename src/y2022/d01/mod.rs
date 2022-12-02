use anyhow::Result;
use snake_helper::unwrap_ok_or;

const INPUT: &str = include_str!("input.txt");
#[allow(unused)]
const TEST_INPUT: &str = include_str!("test_input.txt");

pub fn main() -> Result<()> {
	let out = simple(INPUT);
	println!("simple: {out}");
	println!("extra: {}", extra(INPUT));
	Ok(())
}

#[test]
fn test_input() -> Result<()> {
	assert_eq!(simple(TEST_INPUT), 24000);
	assert_eq!(extra(TEST_INPUT), 45000);
	Ok(())
}
fn extra(input: impl Into<String>) -> u32 {
	let input: String = input.into();
	let input_split: Vec<&str> = input.split("\n\n").collect();
	// dbg!(&input_split);
	let mut biggest: [u32; 3] = [0; 3];
	let mut total: u32;
	for split in input_split {
		total = 0;
		let split = split.split('\n');
		for num in split {
			let num_parsed: u32 = unwrap_ok_or!(num.parse(), e, {
				println!("error for {num:?}: {e}");
				continue;
			});
			total += num_parsed;
		}
		// dbg!(total);
		for big in &mut biggest {
			if total > *big {
				std::mem::swap(&mut (*big), &mut total);
			}
		}
		// dbg!(&biggest);
	}
	dbg!(&biggest);
	let mut biggest_sum = 0;
	biggest.iter().for_each(|b| biggest_sum += b);
	biggest_sum
}

fn simple(input: impl Into<String>) -> u32 {
	let input: String = input.into();
	let input_split: Vec<&str> = input.split("\n\n").collect();
	// dbg!(&input_split);
	let mut biggest: u32 = 0;
	for split in input_split {
		let mut total: u32 = 0;
		let split = split.split('\n');
		for num in split {
			let num_parsed: u32 = unwrap_ok_or!(num.parse(), e, {
				println!("error for {num:?}: {e}");
				continue;
			});
			total += num_parsed;
		}
		if total > biggest {
			biggest = total;
		}
	}
	biggest
}
