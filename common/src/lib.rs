
use std::fs::read_to_string;
use handlebars::Handlebars;

pub mod utils;
pub mod file_finder;



pub fn load_template(path: String, file: String) -> Handlebars<'static> {
	let path_file = format!("{}/{}", path.clone(), file.clone());
	let contents = read_to_string(path_file.clone())
		.unwrap_or_else(|e| panic!("Failed to load template file: {} {}", path_file, e));
	let mut bars = Handlebars::new();
	bars.register_template_string(&file, contents.clone())
		.unwrap_or_else(|e| panic!("Failed to parse template file: {} {}", path_file, e));
	bars
}
