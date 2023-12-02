use crate::utils::load_data;

pub fn part1(path: &str) -> std::io::Result<()> {
	let data = load_data(path)?;

	let total: u32 = data
		.iter()
		.map(|line| {
			let first = line
				.chars()
				.find(|x| x.is_numeric())
				.unwrap()
				.to_digit(10)
				.unwrap();
			let last = line
				.chars()
				.rev()
				.find(|x| x.is_numeric())
				.unwrap()
				.to_digit(10)
				.unwrap();
			return first * 10 + last;
		})
		.sum();

	println!("Total: {total:?}");

	return Ok(());
}

fn take(str: &str, n: usize) -> String {
	return str.chars().take(n).collect();
}

fn part2(path: &str) -> std::io::Result<()> {
	let data = load_data(path)?;

	let mut final_nums: Vec<u32> = vec![];

	for line in data {
		let original_line = line.clone();
		let mut line = line;
		let mut nums: Vec<u32> = vec![];

		while let Some(char) = line.chars().nth(0) {
			let x: Option<u32> = match char {
				'o' => match take(&line, 3).as_str() {
					"one" => Some(1),
					_ => None,
				},
				't' => match take(&line, 3).as_str() {
					"two" => Some(2),
					"thr" => match take(&line, 5).as_str() {
						"three" => Some(3),
						_ => None,
					},
					_ => None,
				},
				'f' => match take(&line, 4).as_str() {
					"four" => Some(4),
					"five" => Some(5),
					_ => None,
				},
				's' => match take(&line, 3).as_str() {
					"six" => Some(6),
					"sev" => match take(&line, 5).as_str() {
						"seven" => Some(7),
						_ => None,
					},
					_ => None,
				},
				'e' => match take(&line, 5).as_str() {
					"eight" => Some(8),
					_ => None,
				},
				'n' => match take(&line, 4).as_str() {
					"nine" => Some(9),
					_ => None,
				},
				ch if char.is_numeric() => Some(ch.to_digit(10).unwrap()),
				_ => None,
			};

			match x {
				Some(num) => nums.push(num),
				_ => {}
			}

			let length = 1;
			line = line[length..].to_string();
		}

		let final_num = *nums.first().unwrap() * 10 + *nums.last().unwrap();
		println!("{original_line:?} - {final_num:?}");
		final_nums.push(final_num);
	}

	let sum = final_nums.iter().sum::<u32>();
	println!("Total: {sum:?}");

	return Ok(());
}

pub fn run() -> std::io::Result<()> {
	println!("--- Part 1 ---");
	part1("./src/day1/input.txt")?;
	println!("--- Part 2 ---");
	part2("./src/day1/input2.txt")?;
	return Ok(());
}
