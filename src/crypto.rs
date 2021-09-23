use std::convert::TryFrom;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::Rng;
use rand::rngs::OsRng;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Entry {
	pub salt: [u8; 22],
	pub nonce: [u8; 12],
	pub ciphertext: Vec<u8>,
}

pub fn encrypt(value: Vec<u8>, password: &str) -> Entry {
	let salt = SaltString::generate(&mut OsRng);

	let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().hash.unwrap();

	let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

	let random_bytes = rand::thread_rng().gen::<[u8; 12]>();
	let nonce = Nonce::from_slice(&random_bytes);

	let encryptedreturn = Entry {
		salt: <[u8; 22]>::try_from(salt.as_bytes()).unwrap(),
		nonce: <[u8; 12]>::try_from(nonce.as_slice()).unwrap(),
		ciphertext: cipher.encrypt(nonce, value.as_slice()).unwrap(),
	};
	return encryptedreturn;
}

pub fn decrypt(encryptedreturn: Entry, password: &str) -> Vec<u8> {
	let nonce = Nonce::from_slice(&encryptedreturn.nonce);
	let password_hash = Argon2::default().hash_password(password.as_bytes(), &String::from_utf8(Vec::from(encryptedreturn.salt)).unwrap()).unwrap().hash.unwrap();
	let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));
	let ciphertext = encryptedreturn.ciphertext;
	let decrypted = cipher.decrypt(nonce, ciphertext.as_slice()).unwrap();
	return decrypted;
}
pub fn store(encryptedreturn: Entry) -> Vec<u8> {
	let mut save = Vec::new();
	save.extend_from_slice(&encryptedreturn.salt);
	save.extend_from_slice(&encryptedreturn.nonce);
	save.extend_from_slice(&encryptedreturn.ciphertext);
	return save;
}