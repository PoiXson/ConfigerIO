
use log::{ Level, log_enabled };
use log::info;

use clap::{ Parser, Subcommand };

use configer_common::utils;
use configer_common::display::{ display_cat, display_diff };

mod configuration;
use crate::configuration::Configuration;

mod playbook;



#[derive(Parser)]
#[clap(author, version, about)]
#[clap(subcommand_required=true, arg_required_else_help=true)]
struct Args {

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

	/// More output per occurrence
	#[clap(short, long, global=true, parse(from_occurrences), display_order=25)]
	verbose: i8,

	/// Less output per occurrence
	#[clap(short, long, global=true, parse(from_occurrences), display_order=26)]
	quiet: i8,

	#[clap(subcommand)]
	command: Option<Commands>,

}



#[derive(Subcommand)]
enum Commands {

	/// Generate config files for bind/named
	#[clap(visible_alias="gen")]
	Generate {

		/// Archive the existing config files
		#[clap(short, long, display_order=11)]
		backup: bool,

		/// Install the generated config files
		#[clap(short, long, display_order=12)]
		install: bool,

		/// Set the path to configer.json
		#[clap(long, takes_value=true, value_name="json", alias="configer-config", display_order=13)]
		configer_file: Option<String>,

		/// Set the path to template files
		#[clap(long, takes_value=true, value_name="path", display_order=14)]
		tpl_path: Option<String>,

	},

}



fn main() {
	let args: Args = Args::parse();
	// log verbose/quiet
	let mut verbosity: i8 = args.verbose - args.quiet;
	{
		let lvl = match verbosity {
			// -qq
			-1=> Level::Error.to_level_filter(),
			// -q
			0 => Level::Warn.to_level_filter(),
			// -v
			1 => Level::Info.to_level_filter(),
			// -vv
			2 => Level::Debug.to_level_filter(),
			_ => {
				// -qqq
				if verbosity < -1 {
					log::LevelFilter::Off
				} else
				// -vvv
				if verbosity > 2 {
					Level::Trace.to_level_filter()
				} else {
					Level::Warn.to_level_filter()
				}
			},
		};
		env_logger::Builder::new()
			.filter_level(lvl)
			.init();
	}
	if log_enabled!(Level::Info) {
		log_panics::init();
	}
	if log_enabled!(Level::Warn) {
		if args.dry   { if log_enabled!(Level::Info) { info!("DRY"  ); } else { println!(" [DRY] "  ); }}
		if args.force { if log_enabled!(Level::Info) { info!("FORCE"); } else { println!(" [FORCE] "); }}
	}
	// handle command
	match &args.command {

		Some(Commands::Generate { backup, install, configer_file, tpl_path }) => {
			// configer.json file
			let cfg_file_str = configer_common::find_configer_file(configer_file);
			// templates path
			let tpl_path_str = configer_common::find_templates_path(tpl_path);
			// load config
			let cfg: Configuration = Configuration::load( cfg_file_str );
			// generate configs
			let book = playbook::generate_configs(cfg, tpl_path_str.clone());
			if *install {
				// backup configs
				if *backup {
todo!("UNFINISHED BACKUP");
				}
				// install configs
todo!("UNFINISHED INSTALL");
			}
			// --cat
			if args.cat {
				display_cat(&book);
			}
			// --diff
			if args.diff {
				display_diff(&book);
			}
		},

		None => { },

	};
}
