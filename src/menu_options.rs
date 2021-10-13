use std::{fs, io};
use std::convert::{Infallible, TryFrom};
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use slpm_file::datatype::DataType;
use slpm_file::header_binary_v0::HeaderBinaryV0;
use slpm_file::header_v0::HeaderV0;
use slpm_file::payload::Entry;

use crate::manager::create_password_entry;
use crate::table_printer::print_table;

pub struct MenuOptions {
	new: bool,
	data_type: DataType,
}

pub fn init(entries: Vec<Entry>) {
	println!("Welcome to SLPM. Choose from following options:");
	println!("1. List existing entries");
	println!("2. Read a password entry");
	println!("3. Create a new password entry");
	println!("4. Remove an existing entry");

	let mut line = "".to_owned();
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	match line.trim() {
		"1" => { list_entries(entries) }
		"3" => { new_password() }
		_ => {}
	}
}

fn list_entries(mut entries: Vec<Entry>) {
	let directory = fs::read_dir("./src/data").unwrap();
	for (i, value) in directory.enumerate() {
		let reader= fs::read(format!("./src/data/{}", value.unwrap().file_name().into_string().unwrap())).unwrap();
		entries.push(Entry::from_bytes(&reader, true));
	}
	let mut headers: Vec<HeaderV0> = vec![];
	for entry in entries {
		headers.push(HeaderV0::from_binary_header(&HeaderBinaryV0::from_bytes(&entry.header)));
	}
	print_table(headers);
	// println!("{}", "Name Type Create-date");
	// for entry in entries {
	// 	let header = HeaderV0::from_binary_header(&HeaderBinaryV0::from_bytes(&entry.header));
	//
	// 	let system_time = UNIX_EPOCH + Duration::from_secs(header.created);
	// 	let datetime = DateTime::<Utc>::from(system_time);
	// 	let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
	//
	// 	println!("{} {:?} {}", header.name.trim(), header.datatype, timestamp_str);
	// }
}

fn new_password() {
	println!("{}", "Provide a name for the password file");
	let mut name = "".to_owned();

	io::stdin()
		.read_line(&mut name)
		.expect("failed to read from stdin");

	println!("{}", "Provide the password to store");
	let mut content = "".to_owned();

	io::stdin()
		.read_line(&mut content)
		.expect("failed to read from stdin");

	println!("{}", "Provide the password used to secure the content");
	let mut key = "".to_owned();

	io::stdin()
		.read_line(&mut key)
		.expect("failed to read from stdin");

	create_password_entry(&name.trim(), content.trim().as_bytes(), key.trim().as_bytes());
}