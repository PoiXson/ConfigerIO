
use log::{ Level, log_enabled, debug };
use clap::{ Parser, Subcommand };

use configer_common::utils;
use configer_common::file_finder::FileFinder;

mod configuration;
use crate::configuration::Configuration;

mod generation;
use crate::generation::cmd_generate;



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
		if args.dry   { println!(" [DRY] "  ); }
		if args.force { println!(" [FORCE] "); }
	}
	if log_enabled!(Level::Info) {
		log_panics::init();
	}
	// handle command
	match &args.command {

		Some(Commands::Generate { backup, install }) => {
			let cfg: Configuration = load_config();
			cmd_generate(
				cfg,
				args.dry, args.force,
				args.cat, args.diff,
				*backup, *install
			);
		},

		None => { },

	};
}



fn load_config() -> Configuration {
	let mut file: String = std::env::var("CONFIGER_CONFIG").unwrap_or("".to_string());
	if file.is_empty() {
		file = FileFinder::new()
			.file("/etc/configer.json" .to_string())
			.file("/var/configer.json" .to_string())
			.file("/configer.json"     .to_string())
			.file("/home/configer.json".to_string())
			.found()
	}
	if file.is_empty() {
		panic!("Failed to find config file");
	}
	debug!("Using config: {}", file);
	if ! std::path::Path::new(&file).is_file() {
		panic!("Config file not found: {}", file);
	}
	Configuration::load(file)
}
