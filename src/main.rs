use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::windows::fs::FileExt;
use std::time::Instant;

use slpm_file::chunk_management::BufferReader;
use slpm_file::datatype::DataType;
use slpm_file::header_binary_v0::HeaderBinaryV0;
use slpm_file::header_v0::HeaderV0;
use slpm_file::payload::Entry;

fn main() {
	test();
}

fn test() {
	let mut old_file = BufferReader::new(File::open("./src/assets/War Thunder 2021.09.27 - 14.44.20.02.DVR_Trim.mp4").unwrap(), 500_000);
	let mut new_file = BufferReader::new(File::create("./src/assets/new.mp4").unwrap(), 500_000);

	let file = &old_file.file;
	old_file.file_len = file.metadata().unwrap().len();
	let buff_count = old_file.file_len / &old_file.buffer_size;

	for _ in 0..=buff_count {
		let result = old_file.read_next();
		new_file = new_file.write_next(result.as_slice());
	}

	assert_eq!(old_file.file.metadata().unwrap().len(), new_file.file.metadata().unwrap().len());
}


fn encrypt_decrypt_regular() {
	let start = Instant::now();
	let header = HeaderBinaryV0::from_parameters(&DataType::File, "Warthunder", None, "new", 100_000);
	let data = fs::read("./src/assets/War Thunder 2021.09.27 - 14.44.20.02.DVR_Trim.mp4").unwrap(); //local, use any
	let entry = Entry::encrypt(&data, &header, "cum");
	fs::write("./src/assets/file.slpm", entry.to_bytes()).unwrap();
	println!("{:?}", start.elapsed());

	let start = Instant::now();
	let read = fs::read("./src/assets/file.slpm").unwrap();
	let serialized = Entry::from_bytes(&read);
	let decrypted = serialized.decrypt("cum");
	let decrypted_header = HeaderBinaryV0::from_bytes(&decrypted.header);
	let serialized_header = HeaderV0::from_binary_header(&decrypted_header);
	println!("{:?}", serialized_header);
	fs::write("./src/assets/", &decrypted.text).unwrap();
	println!("{:?}", start.elapsed());
}