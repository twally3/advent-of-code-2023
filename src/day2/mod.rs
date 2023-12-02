use crate::utils::load_data;

fn part1(path: &str) -> std::io::Result<()> {
	let lines = load_data(path)?;

	let mut sum: u32 = 0;

	for line in lines {
		let (game, rounds) = line.split_once(':').unwrap();

		let (_, game_id) = game.split_once(' ').unwrap();
		let game_id = game_id.parse::<u32>().unwrap();

		let rounds = rounds.split(';').all(|x| {
			x.trim().split(',').all(|x| {
				let (count, colour) = x.trim().split_once(' ').unwrap();
				let count = count.parse::<u32>().unwrap();
				return match colour {
					"red" => count <= 12,
					"green" => count <= 13,
					"blue" => count <= 14,
					_ => panic!("Invalid colour"),
				};
			})
		});

		if rounds {
			sum += game_id;
		}

		println!("{game_id:?} - {rounds:?}");
	}

	println!("Total: {sum:?}");

	return Ok(());
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Round {
	red: u32,
	green: u32,
	blue: u32,
}

fn part2(path: &str) -> std::io::Result<()> {
	let lines = load_data(path)?;

	let mut total: u32 = 0;

	for line in lines {
		let (game, rounds) = line.split_once(':').unwrap();

		let (_, game_id) = game.split_once(' ').unwrap();
		let game_id = game_id.parse::<u32>().unwrap();

		let rounds = rounds.split(';').fold(
			Round {
				red: 0,
				green: 0,
				blue: 0,
			},
			|acc, x| {
				let round = x.trim().split(',').fold(
					Round {
						red: 0,
						green: 0,
						blue: 0,
					},
					|acc, x| {
						let (count, colour) = x.trim().split_once(' ').unwrap();
						let count = count.parse::<u32>().unwrap();
						let mut round = acc;
						match colour {
							"red" => round.red = std::cmp::max(count, round.red),
							"green" => round.green = std::cmp::max(count, round.green),
							"blue" => round.blue = std::cmp::max(count, round.blue),
							_ => {}
						}
						return round;
					},
				);

				return Round {
					red: std::cmp::max(acc.red, round.red),
					green: std::cmp::max(acc.green, round.green),
					blue: std::cmp::max(acc.blue, round.blue),
				};
			},
		);

		let power = rounds.red * rounds.green * rounds.blue;
		total += power;

		println!("{game_id:?} - {rounds:?} - {power:?}");
	}

	println!("Total: {total:?}");

	return Ok(());
}

pub fn run() -> std::io::Result<()> {
	println!("--- Part 1 ---");
	part1("./src/day2/input.txt")?;
	println!("--- Part 2 ---");
	part2("./src/day2/input.txt")?;
	return Ok(());
}
