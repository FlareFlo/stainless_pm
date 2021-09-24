use std::fs;

use crate::processing::header_v0::{DataType, HeaderBinaryV0};
use crate::processing::payload::{Entry};
use chrono::DateTime;

mod processing;


fn main() {
	let header = HeaderBinaryV0::from_parameters(&DataType::Password, "Password", None);
	let entry = Entry::encrypt(b"password2", &header, "cum");
	fs::write("./password2.slpm", entry.to_bytes()).unwrap();

	let read = fs::read("password2.slpm").unwrap();
	let serialized = Entry::from_bytes(&read);
	let decrypted = serialized.decrypt("cum");
	let decrypted_header = HeaderBinaryV0::from_bytes(&decrypted.header);
	println!("{}", String::from_utf8(Vec::from(decrypted_header.name)).unwrap());
	println!("{}", String::from_utf8(decrypted.text).unwrap());
}