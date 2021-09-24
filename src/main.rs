use std::convert::TryInto;
use std::fs;
use std::fs::File;

use crate::crypto::{DataType, decrypt, encrypt, pack_header_from_raw, header_to_bytes};

mod crypto;
mod processing;


fn main() {
	let header = pack_header_from_raw(0, 512, DataType::Password, *b"salt                  ", *b"nonce       ");
	let binary_header = header_to_bytes(header);
	eprintln!("header = {:?}", binary_header.len());
}