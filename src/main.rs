use crate::crypto::{decrypt, encrypt};
use std::fs;
use std::convert::TryInto;
use std::fs::File;
use flate2::read::GzEncoder;
use flate2::Compression;

mod crypto;


fn main() {
	let tar_archive = File::create("password.tar.gz").unwrap();
	let enc = GzEncoder::new(tar_archive, Compression::default());
	let mut tar = tar::Builder::new(enc);
	tar.append_dir_all("./cum2", "./cum").unwrap();
}