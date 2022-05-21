
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



const SERVICE_TITLE: &str = "bind/named";



pub fn generate_configs(cfg: Configuration, tpl_path: String) -> Vec<GenFile> {
	let mut book: Vec<GenFile> = Vec::new();
	info!("Generating configs for {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/named.conf
	{
		let dest_file = "/etc/named.conf".to_string();
		let tpl_file = format!("{}.tpl", safe_file_from_path(dest_file.clone()) );
		let tpl = load_template(tpl_path.clone(), tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
			"internal": cfg.get_internal_hostnames(),
			"external": cfg.get_external_domains(),
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

	// /etc/named/domain.zone
	{
		let tpl_file = "etc-named-domain.zone.tpl".to_string();
		for (domain, zone) in cfg.internal {
			let dest_file = format!("/etc/named/{}.zone", domain);
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

	book
}
