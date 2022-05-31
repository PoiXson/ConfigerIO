
use std::fs::read_to_string;
use std::collections::BTreeMap as Map;
use serde::{ Serialize, Deserialize };



#[derive(Debug, Serialize, Deserialize)]
pub struct DomainDetails {
	pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
	#[serde(default)]
	pub internal: Map<String, DomainDetails>,
	#[serde(default)]
	pub external: Map<String, DomainDetails>,
}



impl Configuration {

	pub fn load(file: String) -> Self {
		let contents = read_to_string(file.clone())
			.unwrap_or_else(|e| panic!("Failed to read configer config file: {} {}", file, e));
		serde_json::from_str(&contents)
			.unwrap_or_else(|e| panic!("Failed to parse configer config file: {} {}", file, e))
	}

}

pub fn get_hostnames(array: &Map<String, DomainDetails>) -> Vec<String> {
	let mut result: Vec<String> = Vec::new();
	for (hostname, _) in array {
		result.push(hostname.clone());
	}
	result
}
