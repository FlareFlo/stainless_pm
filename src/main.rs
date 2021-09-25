use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::os::windows::fs::FileExt;
use std::process::exit;
use std::time::Instant;

use chrono::DateTime;
use sysinfo::{System, SystemExt};

use crate::processing::header_v0::{DataType, HeaderBinaryV0};
use crate::processing::payload::Entry;

mod processing;


fn main() {
	read_file_in_chunks_and_write();
}

fn read_file_in_chunks_and_write() {
	const BUFFER_SIZE: u64 = 524288 * 2;
	const BUFF_U: usize = BUFFER_SIZE as usize;
	let mut file = File::open("./src/assets/old.mp4").unwrap();
	let mut new_file = File::create("./src/assets/new.mp4").unwrap();

	let file_len = file.metadata().unwrap().len();
	let mut offset = 0;
	let buff_count = file_len / BUFFER_SIZE;

	for _ in 0..buff_count {
		let mut buffer = vec![0; BUFF_U];
		let _ = file.seek_read(&mut buffer, offset).unwrap();
		new_file.write(&buffer);
		offset += BUFFER_SIZE;
	}

	let remain = file_len - offset;
	let mut buffer_last = vec![0; remain as usize];
	let _ = file.seek_read(&mut buffer_last, offset).unwrap();
	new_file.write(&buffer_last).unwrap();

	assert_eq!(fs::read("./src/assets/old.mp4").unwrap(), fs::read("./src/assets/new.mp4").unwrap())
}

fn encrypt_decrypt_regular() {
	let start = Instant::now();
	let header = HeaderBinaryV0::from_parameters(&DataType::File, "Warthunder", None);
	let data = fs::read("./src/assets/War Thunder 2021.09.23 - 17.24.16.15.DVR.mp4").unwrap();
	let entry = Entry::encrypt(&data, &header, "cum");
	fs::write("./file.slpm", entry.to_bytes()).unwrap();
	println!("{:?}", start.elapsed());

	let start = Instant::now();
	let read = fs::read("file.slpm").unwrap();
	let serialized = Entry::from_bytes(&read);
	let decrypted = serialized.decrypt("cum");
	let decrypted_header = HeaderBinaryV0::from_bytes(&decrypted.header);
	println!("{}", String::from_utf8(Vec::from(decrypted_header.name.clone())).unwrap());
	fs::write("./decrypted.mp4", &decrypted.text).unwrap();
	println!("{:?}", start.elapsed());
}