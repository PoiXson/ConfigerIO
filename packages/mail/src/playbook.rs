
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



pub const SERVICE_NAME:  &str = "mail";
pub const SERVICE_TITLE: &str = "postfix/dovcot";



pub fn load_templates(tpl_path: String) -> Vec<FileDAO> {
	let mut book: Vec<FileDAO> = Vec::new();
	// /etc/postfix/main.cf
	book.push(FileDAO::new( &tpl_path, "/etc/postfix/main.cf".to_string() ));
	book
}



pub fn generate_configs(_cfg: &Configuration, book: &Vec<FileDAO>) {
	info!("Generating configs for: {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/postfix/main.cf
	{
		let dao = FileDAO::get(&book, "/etc/postfix/main.cf");
		trace!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		let tpl = load_tpl(dao.tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
		});
		render_tpl(&dao, &tpl, &tags);
	}

}
