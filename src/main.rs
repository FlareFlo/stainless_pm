use crate::crypto::{EncryptedReturn, encrypt, decrypt};
use aes_gcm::{Key, Nonce};
use aes_gcm::Aes256Gcm;
use aes_gcm::NewAead;

mod crypto;


fn main() {
	let encrypted = encrypt(Vec::from("cum"), "cheese");

	let decrypted = decrypt(encrypted, "cheese");
	println!("{}", String::from_utf8(decrypted).unwrap());
}