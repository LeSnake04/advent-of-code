crate::base_use!();

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const EXTRA_LOOSE: i32 = ROCK;
const EXTRA_DRAW: i32 = PAPER;
const EXTRA_WIN: i32 = SCISSORS;

pub fn main() {
	let out = run(INPUT, false);
	println!("Simple: {}", out);
	assert_eq!(out, 12645);
	let out2 = run(INPUT, true);
	assert_ne!(out2, 11825);
	println!("Extra: {}", out2)
}

#[test]
fn test() {
	assert_eq!(run(TEST_INPUT, false), 15);
	assert_eq!(run(TEST_INPUT, true), 12);
}

fn run(input: impl Into<String>, extra: bool) -> i32 {
	let input: String = input.into().trim_end().to_string();
	let split = input.split('\n');
	let mut your_score = 0;
	for round in split {
		let signs: Vec<&str> = round.split(' ').collect();
		let enemy = match signs[0] {
			"A" => ROCK,
			"B" => PAPER,
			"C" => SCISSORS,
			i => {
				println!("invalid enemy input {:?}", i);
				continue;
			}
		};
		let you: i32 = match signs[1] {
			"X" => ROCK,
			"Y" => PAPER,
			"Z" => SCISSORS,
			i => {
				println!("invalid you input {:?}", i);
				continue;
			}
		};
		let new_score = if !extra {
			score(enemy, you)
		} else {
			extra_apply(enemy, you)
		};
		your_score += new_score;
	}
	your_score
}

fn score(enemy: i32, you: i32) -> i32 {
	(if enemy == you {
		3
	} else {
		match win(you, enemy) {
			true => 6,
			false => 0,
		}
	}) + you
}

fn win(a: i32, b: i32) -> bool {
	match (a, b) {
		(ROCK, PAPER) => false,
		(ROCK, SCISSORS) => true,
		(PAPER, ROCK) => true,
		(PAPER, SCISSORS) => false,
		(SCISSORS, PAPER) => true,
		(SCISSORS, ROCK) => false,
		_ => unreachable!(),
	}
}

fn extra_apply(enemy: i32, goal: i32) -> i32 {
	match (enemy, goal) {
		(out, EXTRA_DRAW) => out + 3,
		(ROCK, EXTRA_WIN) => PAPER + 6,
		(ROCK, EXTRA_LOOSE) => SCISSORS,
		(PAPER, EXTRA_WIN) => SCISSORS + 6,
		(PAPER, EXTRA_LOOSE) => ROCK,
		(SCISSORS, EXTRA_WIN) => ROCK + 6,
		(SCISSORS, EXTRA_LOOSE) => PAPER,
		_ => unreachable!(),
	}
}
