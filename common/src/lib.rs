
use log::{ info, trace, error };

use std::fs::read_to_string;
use std::io::Write;
use handlebars::Handlebars;

use tempfile::NamedTempFile;

use crate::utils::{
	safe_file_from_path,
	new_temp_file,
};

pub mod utils;
pub mod file_finder;
pub mod display;



pub const DEFAULT_TEMPLATES_PATH: &str = "/usr/share/configer/templates";
pub const DEFAULT_CONFIG_PATHS: &'static [&str] = &[
	"/etc/configer.json",
	"/var/configer.json",
	"/configer.json",
	"/home/configer.json",
];



#[derive(Debug)]
pub struct FileDAO {
	pub dest_file: String,
	pub tpl_file: String,
	pub tmp_file: String,
	pub tmp_handle: NamedTempFile,
}

impl FileDAO {

	pub fn new(dest_file: String, tpl_file_or_path: String) -> Self {
		let tpl_file =
			if tpl_file_or_path.ends_with(".tpl") {
				tpl_file_or_path.clone()
			} else {
				format!("{}/{}.tpl",
					tpl_file_or_path.clone(),
					safe_file_from_path(dest_file.clone()),
				)
			};
		if ! std::path::Path::new(&tpl_file).is_file() {
			panic!("Template file not found: {}", tpl_file.clone());
		}
		let (tmp_file, tmp_handle) = new_temp_file();
		trace!("Temp: {} Rep: {}", tmp_file.clone(), dest_file.clone());
		Self {
			dest_file: dest_file.clone(),
			tpl_file: tpl_file.clone(),
			tmp_file: tmp_file.clone(),
			tmp_handle,
		}
	}

	pub fn get_by_dest<'a>(book: &'a Vec<FileDAO>, dest_file: String) -> &'a FileDAO {
		for dao in book {
			if dao.dest_file == dest_file {
				return &dao;
			}
		}
		panic!("FileDAO not found for: {}", dest_file);
	}

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
						.files(DEFAULT_CONFIG_PATHS.to_vec())
						.found().clone()
				},
			}
		},
	};
	if file.is_empty() {
		error!("File not found: configer.json");
		panic!("Failed to find configer.json file");
	}
	if ! std::path::Path::new(&file).is_file() {
		error!("File not found: {}", file.clone());
		panic!("Config file not found: {}", file.clone());
	}
	info!("Using config: {}", file.clone());
	file.clone()
}

pub fn find_templates_path(arg_path: &Option<String>, service_name: String) -> String {
	// --tpl-path
	let mut path: String = match arg_path {
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
		error!("Path not found to: templates");
		panic!("Failed to find configer templates path");
	}
	if ! std::path::Path::new(&path).is_dir() {
		error!("Path not found to: {}", path.clone());
		panic!("Templates path not found: {}", path.clone());
	}
	// detect service subdir
	{
		let p = format!("{}/{}", &path, &service_name);
		if std::path::Path::new(&p).is_dir() {
			path = p.clone();
		}
	}
	// detect templates/ dir
	{
		let p = format!("{}/templates", &path);
		if std::path::Path::new(&p).is_dir() {
			path = p.clone();
		}
	}
	info!("Using templates: {}", path.clone());
	path.clone()
}



pub fn load_tpl(tpl_file: String) -> Handlebars<'static> {
	let content = read_to_string(tpl_file.clone())
		.unwrap_or_else(|e| panic!("Failed to load template file: {} {}", tpl_file, e));
	let mut tpl = Handlebars::new();
	tpl.register_template_string(&tpl_file, content.clone())
		.unwrap_or_else(|e| panic!("Failed to parse template file: {} {}", tpl_file, e));
	tpl
}



pub fn render_tpl(dao: &FileDAO, tpl: &Handlebars<'static>, tags: &serde_json::Value) -> String {
	let rendered = tpl.render(&dao.tpl_file, &tags)
		.unwrap_or_else(|e| panic!("Failed to render config: {} {}", dao.tpl_file.clone(), e));
	// write temp file
	trace!("Writing file: {}", dao.tmp_file.clone());
	let mut handle = dao.tmp_handle.reopen().unwrap();
	handle.write_all( rendered.as_bytes() ).unwrap();
	rendered
}
