#[macro_export]
macro_rules! base_use {
	() => {
		const INPUT: &str = include_str!("input.txt");
		#[allow(unused)]
		const TEST_INPUT: &str = include_str!("test_input.txt");
	};
}
