use std::fs;

use slpm_file::datatype::DataType;
use slpm_file::header_binary_v0::HeaderBinaryV0;
use slpm_file::payload::Entry;

pub fn create_password_entry(name: &str, content: &[u8], password: &[u8]) {
	let header = HeaderBinaryV0::from_parameters(&DataType::Password, name, None, "", content.len() as u64);
	let entry = Entry::encrypt(content, &header, password);
	fs::write(format!("./src/data/{}.slpm", name), entry.to_bytes()).unwrap();
}

pub fn read_password_entry(bytes: &[u8], password: &str) -> String {
	let entry = Entry::from_bytes(bytes, true);
	let decrypted = entry.decrypt(password);
	String::from_utf8(decrypted.ciphertext).unwrap()
}