use std::convert::TryFrom;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::Rng;
use rand::rngs::OsRng;
use std::any::Any;
use crate::processing::header::Header;


pub struct Entry {
	pub header: [u8; 512],
	pub ciphertext: Vec<u8>,
}

pub fn encrypt(value: Vec<u8>, password: &str) -> Vec<u8> {
	let salt = SaltString::generate(&mut OsRng);

	let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().hash.unwrap();

	let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

	let random_bytes = rand::thread_rng().gen::<[u8; 12]>();
	let nonce = Nonce::from_slice(&random_bytes);

	return cipher.encrypt(nonce, value.as_slice()).unwrap()
}
pub fn decrypt(encrypted: Entry, header: Header, password: &str) -> Vec<u8> {
	let nonce = Nonce::from_slice(&header.nonce);

	let password_hash = Argon2::default().hash_password(password.as_bytes(), &String::from_utf8(Vec::from(header.salt)).unwrap()).unwrap().hash.unwrap();
	let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

	let ciphertext = encrypted.ciphertext;

	return cipher.decrypt(nonce, ciphertext.as_slice()).unwrap();
}

impl Entry {
	// TODO implement header decryption beforehand
	// pub fn decrypt(&self, password: &str) -> Vec<u8>{
	// 	let header =
	// 	let nonce = Nonce::from_slice(&header.nonce);
	//
	// 	let password_hash = Argon2::default().hash_password(password.as_bytes(), &String::from_utf8(Vec::from(header.salt)).unwrap()).unwrap().hash.unwrap();
	// 	let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));
	//
	// 	let ciphertext = encrypted.ciphertext;
	//
	// 	return cipher.decrypt(nonce, ciphertext.as_slice()).unwrap();
	// }
}