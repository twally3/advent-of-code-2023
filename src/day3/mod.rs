use std::collections::HashMap;

use crate::utils::load_data;

fn get_data(arr: &Vec<String>, i: usize, j: usize) -> Option<char> {
	return arr.get(i).and_then(|x| x.chars().nth(j));
}

fn is_symbol(char: Option<char>) -> bool {
	if let Some(char) = char {
		return !char.is_numeric() && char != '.';
	}
	return false;
}

fn thing(lines: &Vec<String>, i: usize, j: usize) -> bool {
	if i != 0 && j != 0 {
		let char = get_data(&lines, i - 1, j - 1);
		if is_symbol(char) {
			return true;
		}
	}
	if i != 0 {
		let char = get_data(&lines, i - 1, j);
		if is_symbol(char) {
			return true;
		}
		let char = get_data(&lines, i - 1, j + 1);
		if is_symbol(char) {
			return true;
		}
	}
	if j != 0 {
		let char = get_data(&lines, i, j - 1);
		if is_symbol(char) {
			return true;
		}
		let char = get_data(&lines, i + 1, j - 1);
		if is_symbol(char) {
			return true;
		}
	}
	let char = get_data(&lines, i, j + 1);
	if is_symbol(char) {
		return true;
	}
	let char = get_data(&lines, i + 1, j);
	if is_symbol(char) {
		return true;
	}
	let char = get_data(&lines, i + 1, j + 1);
	if is_symbol(char) {
		return true;
	}
	return false;
}

fn part1(path: &str) -> std::io::Result<()> {
	let lines = load_data(path)?;

	let mut numbers: Vec<u32> = vec![];

	for (i, line) in lines.iter().enumerate() {
		let mut number: Vec<char> = vec![];
		let mut found = false;

		for (j, char) in line.chars().enumerate() {
			if char.is_numeric() {
				found = found || thing(&lines, i, j);

				number.push(char);
			} else if number.len() != 0 {
				println!("{number:?} - {found:?}");
				if found {
					numbers.push(number.iter().collect::<String>().parse::<u32>().unwrap());
				}
				number = vec![];
				found = false;
			}
		}

		if number.len() != 0 && found {
			println!("{number:?} - {found:?}");
			numbers.push(number.iter().collect::<String>().parse::<u32>().unwrap());
		}
	}

	let sum: u32 = numbers.iter().sum();

	println!("{sum:?}");
	return Ok(());
}

fn find_coords(lines: &Vec<String>, i: usize, j: usize, coords: &mut Vec<(usize, usize)>) {
	let char = get_data(&lines, i, j);
	if char.map_or(false, |x| x.is_numeric()) {
		coords.push((i, j))
	}
}

#[derive(Debug)]
struct Item {
	x0: i32,
	x1: i32,
	y: i32,
	num: u32,
}

fn cartesian_product(item: &Item) -> Vec<(i32, i32)> {
	let mut coords = vec![];
	for y in (item.y - 1)..=(item.y + 1) {
		for x in (item.x0 - 1)..=(item.x1 + 1) {
			if y >= 0 && x >= 0 && (y != item.y || !(item.x0..=item.x1).contains(&x)) {
				coords.push((y, x));
			}
		}
	}
	return coords;
}

fn part2(path: &str) -> std::io::Result<()> {
	let lines = load_data(path)?;

	let mut numbers = vec![];
	let mut gears = vec![];

	for (i, line) in lines.iter().enumerate() {
		let mut num = String::new();
		let mut x0: i32 = 0;
		for (j, char) in line.chars().enumerate() {
			if char.is_numeric() {
				if num.is_empty() {
					x0 = j as i32;
				}
				num.push(char);
			} else {
				if char == '*' {
					gears.push((i as i32, j as i32));
				}
				if !num.is_empty() {
					numbers.push(Item {
						x0,
						x1: j as i32 - 1,
						y: i as i32,
						num: num.parse().unwrap(),
					});
					num.clear();
					x0 = 0;
				}
			}
		}

        if !num.is_empty() {
            numbers.push(Item {
                x0,
                x1: line.len() as i32 - 1,
                y: i as i32,
                num: num.parse().unwrap(),
            })
        }
	}

	let mut gears = gears
		.iter()
		.map(|&x| (x, vec![]))
		.collect::<HashMap<(i32, i32), Vec<u32>>>();

	for number in numbers.into_iter() {
		for n in cartesian_product(&number) {
			if let Some(v) = gears.get_mut(&n) {
				v.push(number.num);
			}
		}
	}

	let total: u32 = gears
		.into_iter()
		.filter_map(|(_k, v)| {
			if v.len() == 2 {
				return Some(v[0] * v[1]);
			} else {
				return None;
			};
		})
		.sum();

	println!("Total: {total:?}");

	return Ok(());
}

pub fn run() -> std::io::Result<()> {
	println!("--- Part 1 ---");
	part1("./src/day3/input.txt")?;
	println!("--- Part 2 ---");
	part2("./src/day3/input.txt")?;
	return Ok(());
}
