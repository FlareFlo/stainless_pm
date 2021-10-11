use std::{fs, io};
use std::convert::{Infallible, TryFrom};

use slpm_file::datatype::DataType;

use crate::manager::create_password_entry;

pub struct MenuOptions {
	new: bool,
	data_type: DataType,
}

pub fn init() {
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
		"1" => { list_entries() }
		"2" => { new_password() }
		_ => {}
	}
}

fn list_entries() {
	let directory = fs::read_dir("./src/data").unwrap();
	for (i, value) in directory.enumerate() {
		println!("{:?}", value.unwrap().file_name());
	}
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