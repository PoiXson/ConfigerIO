
use log::{ debug };
use std::fs::read_to_string;
use handlebars::Handlebars;

pub mod utils;
pub mod file_finder;
pub mod display;



//TODO
pub const DEFAULT_PATH_TEMPLATES: &str = "/zcode/web/ConfigerIO/packages/dns/templates";



#[derive(Debug)]
pub struct GenFile {
	pub dest_file: String,
	pub tpl_file: String,
	pub rendered: String,
}



pub fn load_template(tpl_path: String, tpl_file: String) -> Handlebars<'static> {
	let file = format!("{}/{}", tpl_path.clone(), tpl_file.clone());
	let contents = read_to_string(file.clone())
		.unwrap_or_else(|e| panic!("Failed to load template file: {} {}", file, e));
	let mut tpl = Handlebars::new();
	tpl.register_template_string(&tpl_file, contents.clone())
		.unwrap_or_else(|e| panic!("Failed to parse template file: {} {}", file, e));
	tpl
}



pub fn find_configer_config() -> String {
	let mut file: String = std::env::var("CONFIGER_CONFIG")
		.unwrap_or("".to_string());
	if file.is_empty() {
		file = file_finder::FileFinder::new()
			.file("/etc/configer.json" .to_string())
			.file("/var/configer.json" .to_string())
			.file("/configer.json"     .to_string())
			.file("/home/configer.json".to_string())
			.found()
	}
	if file.is_empty() {
		panic!("Failed to find configer config file");
	}
	debug!("Using config: {}", file);
	if ! std::path::Path::new(&file).is_file() {
		panic!("Config file not found: {}", file);
	}
	file
}
