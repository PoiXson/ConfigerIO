
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



pub const SERVICE_NAME:  &str = "dns";
pub const SERVICE_TITLE: &str = "bind/named";



pub fn load_templates(tpl_path: String) -> Vec<FileDAO> {
	let mut book: Vec<FileDAO> = Vec::new();
	// /etc/named.conf
	book.push(FileDAO::new( &tpl_path, "/etc/named.conf".to_string() ));
	book
}



pub fn generate_configs(cfg: &Configuration, book: &Vec<FileDAO>) {
	info!("Generating configs for: {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/named.conf
	{
		let dao = FileDAO::get(&book, "/etc/named.conf");
		trace!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		let tpl = load_tpl(dao.tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
			"internal": cfg.get_internal_hosts(),
			"external": cfg.get_external_hosts(),
		});
		render_tpl(&dao, &tpl, &tags);
	}

}
