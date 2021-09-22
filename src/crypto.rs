use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead, AeadMut};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use rand::Rng;
use aes_gcm::aes::cipher::generic_array::sequence::GenericSequence;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::AeadCore;
use std::convert::TryFrom;

pub(crate) struct Entry {
	openname: String,
	closedname: Option<String>,
}

pub struct EncryptedReturn {
	pub cipher: Aes256Gcm,
	pub ciphertext: Vec<u8>,
	pub salt: SaltString,
	pub nonce: [u8; 12],
}

pub fn encrypt(value: Vec<u8>, password: &str) -> EncryptedReturn {
	let salt = SaltString::generate(&mut OsRng);

	let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().hash.unwrap();

	let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

	let random_bytes = rand::thread_rng().gen::<[u8; 12]>();
	let nonce = Nonce::from_slice(&random_bytes);

	let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
		.expect("encryption failure!"); // NOTE: handle this error to avoid panics!

	let encryptedreturn = EncryptedReturn {
		cipher,
		ciphertext: cipher.encrypt(nonce, value.as_slice()).unwrap(),
		salt,
		nonce: <[u8; 12]>::try_from(nonce.as_slice()).unwrap()
	};
	return encryptedreturn
}

pub fn decrypt(encryptedreturn: EncryptedReturn) -> Vec<u8>{
	let nonce = Nonce::from_slice(&encryptedreturn.nonce);
	let cipher = encryptedreturn.cipher;
	let ciphertext = encryptedreturn.ciphertext;
	let decrypted = cipher.decrypt(nonce, ciphertext).unwrap();
	return decrypted
}