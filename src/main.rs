mod day1;

fn main() -> std::io::Result<()> {
	// let pattern = std::env::args().nth(1).expect("no pattern given");
	let pattern = String::from("day1");
	return match pattern.as_str() {
		"day1" => day1::run(),
		_ => Err(std::io::Error::new(
			std::io::ErrorKind::InvalidInput,
			"Invalid input".to_owned(),
		)),
	};
}
