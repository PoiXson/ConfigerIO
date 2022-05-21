
use std::fs::read_to_string;
use std::collections::BTreeMap as Map;
use serde::{ Serialize, Deserialize };



#[derive(Debug, Serialize, Deserialize)]
pub struct SiteDomain {
	pub domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
	pub sites: Map<String, SiteDomain>,
}



impl Configuration {

	pub fn load(file: String) -> Self {
		let contents = read_to_string(file.clone())
			.unwrap_or_else(|e| panic!("Failed to read configer config file: {} {}", file, e));
		serde_json::from_str(&contents)
			.unwrap_or_else(|e| panic!("Failed to parse configer config file: {} {}", file, e))
	}

/*
	pub fn get_internal_hostnames(&self) -> Vec<String> {
		let mut array: Vec<String> = Vec::new();
		for key in self.internal.keys() {
			array.push(key.clone());
		}
		array
	}

	pub fn get_external_domains(&self) -> Vec<String> {
		let mut array: Vec<String> = Vec::new();
		for key in self.external.keys() {
			array.push(key.clone());
		}
		array
	}
*/

}
