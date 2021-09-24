use std::fs;

use crate::processing::header_v0::{DataType, HeaderBinaryV0};
use crate::processing::payload::{Entry};

mod processing;


fn main() {
	let header = HeaderBinaryV0::pack_header_from_parameters(0, &DataType::Password);
	let encrypted = Entry::encrypt(b"password", &header, "cum");
	fs::write("./password.slpm", encrypted.create_file()).unwrap();

	let file = fs::read("password.slpm").unwrap();
	let entry = Entry::serialize_from_file(&file);
	let decrypted = entry.decrypt("cum");
	println!("{:?}", String::from_utf8(decrypted).unwrap());
}