pub fn load_data(path: &str) -> Result<Vec<String>, std::io::Error> {
	return std::fs::read_to_string(path)
		.map(|x| x.lines().map(|x| x.to_owned()).collect::<Vec<_>>());
}
