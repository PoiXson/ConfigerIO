
use std::fs::read_to_string;
use std::collections::BTreeMap as Map;
use serde::{ Serialize, Deserialize };



#[derive(Debug, Serialize, Deserialize)]
pub struct SiteDetails {
	pub domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
	pub sites: Map<String, SiteDetails>,
}



impl Configuration {

	pub fn load(file: String) -> Self {
		let contents = read_to_string(file.clone())
			.unwrap_or_else(|e| panic!("Failed to read configer config file: {} {}", file, e));
		serde_json::from_str(&contents)
			.unwrap_or_else(|e| panic!("Failed to parse configer config file: {} {}", file, e))
	}

}
