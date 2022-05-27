
use log::{ Level, log_enabled, info };

use clap::{ Parser, Subcommand };

use configer_common::utils;
use configer_common::find_configer_config;
use configer_common::display::{ display_cat, display_diff };

mod configuration;
use crate::configuration::Configuration;

mod playbook;



#[derive(Parser)]
#[clap(author, version, about)]
#[clap(subcommand_required=true, arg_required_else_help=true)]
struct Args {

	#[clap(flatten)]
	verbose: clap_verbosity_flag::Verbosity,

	/// Perform a trial run with no changes made
	#[clap(short, long, global=true, display_order=1)]
	dry: bool,

	/// Use newly generated config without testing
	#[clap(short, long, global=true, display_order=2)]
	force: bool,

	/// Display the generated config files
	#[clap(short='C', long, global=true, display_order=3)]
	cat: bool,

	/// Display differences between the existing and newly generated config files
	#[clap(short='D', long, global=true, display_order=4)]
	diff: bool,

	#[clap(subcommand)]
	command: Option<Commands>,

}



#[derive(Subcommand)]
enum Commands {

	/// Generate config files for bind/named
	#[clap(visible_alias="gen")]
	Generate {

		/// Archive the existing config files
		#[clap(short, long, display_order=1)]
		backup: bool,

		/// Install the generated config files
		#[clap(short, long, display_order=2)]
		install: bool,

	},

}



fn main() {
	let args: Args = Args::parse();
	// modified log level
	utils::modified_verbose_level( args.verbose.log_level() );
	if log_enabled!(Level::Info) {
		log_panics::init();
	}
	if log_enabled!(Level::Warn) {
		if args.dry   { if log_enabled!(Level::Info) { info!("DRY"  ); } else { println!(" [DRY] "  ); }}
		if args.force { if log_enabled!(Level::Info) { info!("FORCE"); } else { println!(" [FORCE] "); }}
	}
	// handle command
	match &args.command {

		Some(Commands::Generate { backup, install }) => {
if *backup || *install {
	todo!("UNFINISHED");
}
			let path_tpl = configer_common::DEFAULT_PATH_TEMPLATES.to_string();
			// load config
			let cfg: Configuration =
				Configuration::load(
					find_configer_config()
				);
			let book = playbook::generate_configs(cfg, path_tpl.clone());
			// --diff
			if args.diff {
				display_diff(&book);
			}
			// --cat
			if args.cat {
				display_cat(&book);
			}
		},

		None => { },

	};
}