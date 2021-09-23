use crate::crypto::{decrypt, encrypt, store, load};
use std::fs;
use std::convert::TryInto;
use std::fs::File;

mod crypto;


fn main() {
	// let yes = encrypt(Vec::from("yes"), "cum");
	// let save = store(yes);
	// fs::write("./save.slpm", &save).unwrap();
	// println!("{:?}", save);
	let file = fs::read("save.slpm").unwrap();
	let file_loaded = load(file);
	let decrypted = decrypt(file_loaded, "cum");
	eprintln!("decrypted = {:?}", String::from_utf8(decrypted));
}