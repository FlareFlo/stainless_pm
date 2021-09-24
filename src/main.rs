use std::convert::TryInto;
use std::fs;
use std::fs::File;

use crate::processing::header_v0::{DataType, HeaderV0, HeaderBinaryV0};
use crate::processing::payload::{Entry};

mod processing;


fn main() {
	let header = HeaderBinaryV0::pack_header_from_raw(0, DataType::Password);
	let encrypted = Entry::encrypt(Vec::from("password"), header, "cum");
	fs::write("./password.slpm", encrypted.create_file()).unwrap();

	let file = fs::read("password.slpm").unwrap();
	let entry = Entry::read_from_file(file);
	let decrypted = entry.decrypt("cum");
	println!("{:?}", String::from_utf8(decrypted).unwrap());
}