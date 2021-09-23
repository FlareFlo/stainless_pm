use crate::crypto::{decrypt, encrypt, store};
use std::fs;
use std::convert::TryInto;
use std::fs::File;

mod crypto;


fn main() {
	let yes = encrypt(Vec::from("yes"), "cum");
	let save = store(yes);
	println!("{:?}", save);
}