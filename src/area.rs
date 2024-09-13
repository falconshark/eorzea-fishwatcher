use std::fs::File;
use std::io::BufReader;

pub fn get_version_list() -> std::io::Result<serde_json::Value>{
	let file = File::open("version.json").unwrap();
	let reader = BufReader::new(file);
	let json: serde_json::Value = serde_json::from_reader(reader)?;
	Ok(json)
}

pub fn get_area_list() -> std::io::Result<serde_json::Value>{
	let file = File::open("area.json").unwrap();
	let reader = BufReader::new(file);
	let json: serde_json::Value = serde_json::from_reader(reader)?;
	Ok(json)
}