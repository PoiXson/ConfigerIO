
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



pub const SERVICE_NAME:  &str = "php";
pub const SERVICE_TITLE: &str = "php-fpm";



pub fn load_templates(cfg: &Configuration, tpl_path: String) -> Vec<FileDAO> {
	let mut book: Vec<FileDAO> = Vec::new();
	// /etc/php.ini
	book.push(FileDAO::new(
		"/etc/php.ini".to_string(),
		tpl_path.clone()
	));
	// /etc/php-fpm.d/<user>.conf
	for (user, _) in &cfg.sites {
		let dest_file = format!("/etc/php-fpm.d/{}.conf", user.clone()).to_string();
		let tpl_file = format!("{}/etc-php-fpm.d-user.conf.tpl", tpl_path);
		book.push(FileDAO::new(
			dest_file.clone(),
			tpl_file.clone(),
		));
	}
	book
}



pub fn generate_configs(cfg: &Configuration, book: &Vec<FileDAO>) {
	info!("Generating configs for: {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/php.ini
	{
		let dao = FileDAO::get_by_dest(&book, "/etc/php.ini".to_string());
		debug!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		let tpl = load_tpl(dao.tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
		});
		render_tpl(&dao, &tpl, &tags);
	}

	// /etc/php-fpm.d/<user>.conf
	{
		for (user, details) in &cfg.sites {
			let dest_file = format!("/etc/php-fpm.d/{}.conf", user.clone()).to_string();
			let dao = FileDAO::get_by_dest(&book, dest_file.clone());
			debug!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
			let tpl = load_tpl(dao.tpl_file.clone());
			let tags = json!({
				"timestamp": timestamp.clone(),
				"user":    user.clone(),
				"details": &details,
				"hostname": &details.domain.clone(),
			});
			render_tpl(&dao, &tpl, &tags);
		}
	}

}
