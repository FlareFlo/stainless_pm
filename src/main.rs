use crate::crypto::{decrypt, encrypt};
use std::fs;
use std::convert::TryInto;
use std::fs::File;

mod crypto;


fn main() {
	let yes = encrypt(Vec::from("yeadasdasdsadsadsadss"), "cum");
}