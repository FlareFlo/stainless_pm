use std::convert::TryInto;
use std::fs;
use std::fs::File;

use crate::crypto::{decrypt, encrypt};
use crate::processing::header::{DataType, Header, HeaderBinary};

mod crypto;
mod processing;


fn main() {
	// let header = HeaderBinary::pack_header_from_raw(0, 512, DataType::Password, *b"salt                  ", *b"nonce       ").header_to_bytes();
	// fs::write("./header.slpmh", header).unwrap();
	let raw_header = fs::read("header.slpmh").unwrap();
	let header_binary = HeaderBinary::deserialize_binary_v0(raw_header).unpack_header();
	eprintln!("header_binary = {:?}", header_binary);
}