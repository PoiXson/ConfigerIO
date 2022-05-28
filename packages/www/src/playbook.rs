
use log::{ info, trace };

use serde_json::json;

use configer_common::FileDAO;
use configer_common::{
	load_tpl,
	render_tpl,
};
use configer_common::utils::{
	get_timestamp,
};

use crate::Configuration;



pub const SERVICE_NAME:  &str = "www";
pub const SERVICE_TITLE: &str = "nginx";



pub fn load_templates(tpl_path: String) -> Vec<FileDAO> {
	let mut book: Vec<FileDAO> = Vec::new();
	// /etc/nginx/nginx.conf
	book.push(FileDAO::new( &tpl_path, "/etc/nginx/nginx.conf".to_string() ));
	book
}



pub fn generate_configs(_cfg: &Configuration, book: &Vec<FileDAO>) {
	info!("Generating configs for: {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/nginx/nginx.conf
	{
		let dao = FileDAO::get(&book, "/etc/nginx/nginx.conf");
		trace!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		let tpl = load_tpl(dao.tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
		});
		render_tpl(&dao, &tpl, &tags);
	}

}
