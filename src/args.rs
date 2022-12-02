use clap::{command, value_parser, Arg, ArgMatches};

pub fn matches() -> ArgMatches {
	command!()
		.args(&[Arg::new("day")
			.short('d')
			.long("day")
			.value_parser(value_parser!(u8))])
		.get_matches()
}
