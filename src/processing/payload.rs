use std::convert::TryFrom;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::Rng;
use rand::rngs::OsRng;
use std::any::Any;
use crate::processing::header_v0::{HeaderV0, HeaderBinaryV0};


pub struct Entry {
	pub header: [u8; 512],
	pub salt: [u8; 22],
	pub nonce: [u8; 12],
	pub ciphertext: Vec<u8>,
}

impl Entry {
	pub fn encrypt(value: Vec<u8>, header: HeaderBinaryV0, password: &str) -> Self {
		let salt = SaltString::generate(&mut OsRng);

		let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().hash.unwrap();

		let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

		let random_bytes = rand::thread_rng().gen::<[u8; 12]>();
		let nonce = Nonce::from_slice(&random_bytes);

		let entry = Entry {
			header: <[u8; 512]>::try_from(header.header_to_bytes()).unwrap(),
			salt: <[u8; 22]>::try_from(salt.as_bytes()).unwrap(),
			nonce: <[u8; 12]>::try_from(nonce.as_slice()).unwrap(),
			ciphertext: cipher.encrypt(nonce, value.as_slice()).unwrap(),
		};

		return entry
	}

	pub fn decrypt(&self, password: &str) -> Vec<u8>{
		let nonce = Nonce::from_slice(&self.nonce);

		let password_hash = Argon2::default().hash_password(password.as_bytes(), &String::from_utf8(Vec::from(self.salt)).unwrap()).unwrap().hash.unwrap();
		let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

		let ciphertext = &self.ciphertext;

		return cipher.decrypt(nonce, ciphertext.as_slice()).unwrap();
	}
	pub fn read_from_file(file: Vec<u8>) -> Self {
		let header_and_rest = file.split_at(512);
		let salt_and_rest = header_and_rest.1.split_at(22);
		let nonce_and_rest = salt_and_rest.1.split_at(12);
		let entry = Self {
			header: <[u8; 512]>::try_from(header_and_rest.0).unwrap(),
			salt: <[u8; 22]>::try_from(salt_and_rest.0).unwrap(),
			nonce: <[u8; 12]>::try_from(nonce_and_rest.0).unwrap(),
			ciphertext: Vec::from(nonce_and_rest.1)
		};
		return entry
	}
	pub fn create_file(&self) -> Vec<u8> {
		let mut file = Vec::new();
		file.extend_from_slice(&self.header);
		file.extend_from_slice(&self.salt);
		file.extend_from_slice(&self.nonce);
		file.extend_from_slice(&self.ciphertext);
		return file
	}
}