use std::fs;
use std::time::Instant;
use slpm_file::header_v0::HeaderV0;
use slpm_file::header_binary_v0::HeaderBinaryV0;
use slpm_file::payload::Entry;
use slpm_file::datatype::DataType;

fn main() {
	encrypt_decrypt_regular();
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