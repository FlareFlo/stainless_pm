use std::str::from_utf8;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordHash};
use rand::rngs::OsRng;


fn main() {
	let mut password = "super-secret-password".to_owned();
	if password.len() != 32 {
		password = format!("{}{}", password, " ".repeat(32 - password.len()));
	}

	let key = Key::from_slice(password.as_bytes());
	let cipher = Aes256Gcm::new(key);

	let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

	let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
		.expect("encryption failure!"); // NOTE: handle this error to avoid panics!

	let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
		.expect("decryption failure!"); // NOTE: handle this error to avoid panics!

	assert_eq!(&plaintext, b"plaintext message");
}