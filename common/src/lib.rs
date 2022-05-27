
use log::debug;

use std::fs::read_to_string;
use handlebars::Handlebars;

pub mod utils;
pub mod file_finder;
pub mod display;



pub const DEFAULT_TEMPLATES_PATH: &str = "/usr/share/configer/templates";



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



pub fn find_configer_file(arg_file: &Option<String>) -> String {
	// --configer-file
	let file: String = match arg_file {
		Some(f) => f.clone(),
		_ => {
			// env CONFIGER_FILE
			match std::env::var("CONFIGER_FILE") {
				Ok(f) => f.clone(),
				Err(_) => {
					// default paths
					file_finder::FileFinder::new()
						.file("/etc/configer.json" .to_string())
						.file("/var/configer.json" .to_string())
						.file("/configer.json"     .to_string())
						.file("/home/configer.json".to_string())
						.found().clone()
				},
			}
		},
	};
	if file.is_empty() {
		panic!("Failed to find configer.json file");
	}
	if ! std::path::Path::new(&file).is_file() {
		panic!("Config file not found: {}", file.clone());
	}
	debug!("Using config: {}", file.clone());
	file.clone()
}

pub fn find_templates_path(arg_path: &Option<String>) -> String {
	// --tpl-path
	let path: String = match arg_path {
		Some(p) => p.clone(),
		_ => {
			// env CONFIGER_TPL_PATH
			match std::env::var("CONFIGER_TPL_PATH") {
				Ok(p) => p.clone(),
				Err(_) => {
					// default path
					DEFAULT_TEMPLATES_PATH.to_string()
				}
			}
		},
	};
	if path.is_empty() {
		panic!("Failed to find configer templates path");
	}
	if ! std::path::Path::new(&path).is_dir() {
		panic!("Templates path not found: {}", path.clone());
	}
	debug!("Using templates: {}", path.clone());
	path.clone()
}
