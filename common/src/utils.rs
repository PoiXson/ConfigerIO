
use log::Level;

use tempfile::NamedTempFile;



pub fn modified_verbose_level(val: Option<Level>) {
	let lvl = match val {
		// default
		Some(log::Level::Error) => log::Level::Warn.to_level_filter(),
		// -v
		Some(log::Level::Warn)  => log::Level::Info.to_level_filter(),
		// -vv
		Some(log::Level::Info)  => log::Level::Debug.to_level_filter(),
		// -vvv
		Some(log::Level::Debug) => log::Level::Trace.to_level_filter(),
		// -vvvv
		Some(log::Level::Trace) => log::Level::Trace.to_level_filter(),
		None => log::LevelFilter::Off,
	};
	env_logger::Builder::new()
		//.filter_level(args.verbose.log_level_filter())
		.filter_level(lvl)
		.init();
}



/// Makes a safe filename from a path
/// /etc/some.file -> etc-some.file
pub fn safe_file_from_path(file: String) -> String {
	let mut f:String = file.clone();
	if '/' == f.chars().take(1).last().unwrap() {
		f.remove(0);
	}
	f.chars().map(|x|
		match x {
			'A'..='Z' => x,
			'a'..='z' => x,
			'.' | '-' | '_' => x,
			_ => '-',
		}).collect()
}
#[test]
fn test_safe_file_from_path() {
	assert_eq!( safe_file_from_path("/etc/some.file".to_string()), "etc-some.file" );
}



/// Creates a new temp file /tmp/tmp.xxxxxxxxxx
pub fn new_temp_file() -> (String, NamedTempFile) {
	let tmp: NamedTempFile =
		tempfile::Builder::new()
			.prefix("tmp.")
			.rand_bytes(10)
			.tempfile().unwrap();
	( tmp.path().display().to_string(), tmp )
}



pub fn get_timestamp() -> String {
	let now = chrono::Local::now();
	now.to_rfc2822()
}
