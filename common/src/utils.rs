
use tempfile::NamedTempFile;



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



pub fn remove_white_space_lines(source: String) -> String {
	let mut result = String::new();
	let mut first = true;
	for line in source.lines() {
		if first { first = false; } else {
			result.push_str("\n");
		}
		result.push_str( line.trim() );
	}
	result
}

pub fn remove_head_comments(source: String) -> (u32, String) {
	let mut result = String::new();
	let mut first = true;
	let mut head = true;
	let mut removed: u32 = 0;
	'LINES_LOOP:
	for line in source.lines() {
		if head {
			if line.is_empty()
			|| line.starts_with("//")
			|| line.starts_with("#")
			|| line.starts_with(";") {
				removed += 1;
				continue 'LINES_LOOP;
			}
			head = false;
		}
		if first { first = false; } else {
			result.push_str("\n");
		}
		result.push_str(line);
	}
	(removed, result)
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
	chrono::Local::now()
		.to_rfc2822()
}
