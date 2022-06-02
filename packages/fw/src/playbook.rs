
use log::{ info, debug };

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



pub const SERVICE_NAME:  &str = "fw";
pub const SERVICE_TITLE: &str = "fw";



pub fn load_templates(cfg: &Configuration, tpl_path: String) -> Vec<FileDAO> {
	let mut book: Vec<FileDAO> = Vec::new();
	// /etc/sysconfig/iptables
	book.push(FileDAO::new(
		"/etc/sysconfig/iptables".to_string(),
		tpl_path.clone()
	));
	book
}



pub fn generate_configs(cfg: &Configuration, book: &Vec<FileDAO>) {
	info!("Generating configs for: {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/sysconfig/iptables
	{
		let dao = FileDAO::get_by_dest(&book, "/etc/sysconfig/iptables".to_string());
		debug!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		let tpl = load_tpl(dao.tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
		});
		render_tpl(&dao, &tpl, &tags);
	}

}
