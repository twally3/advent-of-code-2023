mod day1;
mod day2;
mod utils;

fn main() -> std::io::Result<()> {
	let pattern = std::env::args().nth(1).expect("no pattern given");
	return match pattern.as_str() {
		"day1" => day1::run(),
		"day2" => day2::run(),
		_ => Err(std::io::Error::new(
			std::io::ErrorKind::InvalidInput,
			"Invalid input".to_owned(),
		)),
	};
}
