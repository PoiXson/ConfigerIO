
use log::error;
use colored::Colorize;
use std::fs::read_to_string;
use similar::{ TextDiff, ChangeTag };

use crate::utils::{
	remove_white_space_lines,
	remove_head_comments,
};

use crate::FileDAO;



pub fn display_cat(book: &Vec<FileDAO>) {
	for dao in book.iter() {
		println!();
		println!(" ########################################");
		println!(" ### FILE: {}", dao.dest_file.clone());
		println!(" ### TEMP: {}", dao.tmp_file.clone());
		let rendered = match read_to_string(dao.tmp_file.clone()) {
			Ok(d) => d,
			Err(_) => {
				error!("Missing file: {}", dao.tmp_file.clone());
				error!("Representing: {}", dao.dest_file.clone());
				panic!("Missing: {} Representing: {}", dao.tmp_file.clone(), dao.dest_file.clone());
			},
		};
		println!("{}", rendered);
	}
	println!();
}



pub fn display_diff(book: &Vec<FileDAO>) {
	'BOOK_LOOP:
	for dao in book.iter() {
		println!();
		let original = match read_to_string(dao.dest_file.clone()) {
			Ok(d) => d,
			Err(_) => {
				println!("{} {}", format!("Missing:").red(), dao.dest_file.clone());
				continue 'BOOK_LOOP;
			},
		};
		let rendered = match read_to_string(dao.tmp_file.clone()) {
			Ok(d) => d,
			Err(_) => {
				println!("{} {}", format!("Missing:").red(), dao.dest_file.clone());
				continue 'BOOK_LOOP;
			},
		};
		// compare files
		let (mut ln_a, diff_a) = remove_head_comments(remove_white_space_lines(original.clone()));
		let (mut ln_b, diff_b) = remove_head_comments(remove_white_space_lines(rendered.clone()));
		let diff = TextDiff::from_lines(&diff_a, &diff_b);
		let mut diffs: Vec<(char, u32, String)> = Vec::new();
		let mut count_adds: u32 = 0;
		let mut count_rems: u32 = 0;
		'CHANGE_LOOP:
		for change in diff.iter_all_changes() {
			match change.tag() {
				ChangeTag::Equal => {
					ln_a += 1; ln_b += 1;
					continue 'CHANGE_LOOP;
				},
				ChangeTag::Insert => {
					ln_b += 1; count_adds += 1;
					diffs.push(( '+', ln_b, change.to_string() ));
				},
				ChangeTag::Delete => {
					ln_a += 1; count_rems += 1;
					diffs.push(( '-', ln_a, change.to_string() ));
				},
			};
		} // end CHANGE_LOOP
		if count_adds == 0 && count_rems == 0 {
			println!(
				"{} {}",
				"Unchanged:".green(),
				dao.dest_file.clone()
			);
		} else {
			println!(
				"{} [+{}/-{}] {}",
				"Changed:".red(),
				count_adds, count_rems,
				dao.dest_file.clone()
			);
		}
		for (d, ln, line) in diffs {
			print!(
				"{} {}",
				if d == '+' { format!("[{}{}{}]", d, ln, d).green()
				} else {      format!("[{}{}{}]", d, ln, d).red() },
				line
			);
		}
	} // end BOOK_LOOP
	println!();
}
