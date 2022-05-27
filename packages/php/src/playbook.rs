
use log::info;
use serde_json::json;

use configer_common::{
	GenFile,
	load_template,
};
use configer_common::utils::{
	get_timestamp,
	safe_file_from_path,
};

use crate::configuration::Configuration;



pub const SERVICE_NAME:  &str = "php";
pub const SERVICE_TITLE: &str = "php-fpm";



pub fn generate_configs(cfg: Configuration, tpl_path: String) -> Vec<GenFile> {
	let mut book: Vec<GenFile> = Vec::new();
	info!("Generating configs for {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/php.ini
	{
		let dest_file = "/etc/php.ini".to_string();
		let tpl_file = format!("{}.tpl", safe_file_from_path(dest_file.clone()) );
		let tpl = load_template(tpl_path.clone(), tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
		});
		let rendered = tpl.render(&tpl_file, &tags)
			.unwrap_or_else(|e| panic!("Failed to render config: {} {}", tpl_file, e));
		book.push(
			GenFile {
				dest_file: dest_file.clone(),
				tpl_file:  tpl_file.clone(),
				rendered,
			}
		);
		info!("Generated: {}", dest_file.clone());
	}

	// /etc/php-fpm.conf
	{
		let dest_file = "/etc/php-fpm.conf".to_string();
		let tpl_file = format!("{}.tpl", safe_file_from_path(dest_file.clone()) );
		let tpl = load_template(tpl_path.clone(), tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
//			"internal": cfg.get_internal_hostnames(),
//			"external": cfg.get_external_domains(),
		});
		let rendered = tpl.render(&tpl_file, &tags)
			.unwrap_or_else(|e| panic!("Failed to render config: {} {}", tpl_file, e));
		book.push(
			GenFile {
				dest_file: dest_file.clone(),
				tpl_file:  tpl_file.clone(),
				rendered,
			}
		);
		info!("Generated: {}", dest_file.clone());
	}

/*
	// /etc/php-fpm.d/user.conf
	{
		let tpl_file = "etc-php-fpm.d-user.conf.tpl".to_string();
		for (user, site in cfg.sites {
			let dest_file = format!("/etc/php-fpm.d/{}.conf", user);
//TODO
			book.push(
				GenFile {
					dest_file: dest_file.clone(),
					tpl_file: tpl_file.to_string(),
					rendered: "".to_string(),
				}
			);
		}
	}
*/

	book
}
