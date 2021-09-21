use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use rand::Rng;
use aes_gcm::aes::cipher::generic_array::sequence::GenericSequence;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::AeadCore;

struct Entry {
	openname: String,
	closedname: Option<String>,
}

struct EncryptedReturn<T> {
	value: Vec<u8>,
	salt: SaltString,
	nonce: GenericArray<u8, NonceSize<>>,
}

impl Entry {
	fn encrypt(value: Vec<u8>, password: &str) -> EncryptedReturn<T> {
		let password = password.as_bytes();
		let salt = SaltString::generate(&mut OsRng);

		let argon2 = Argon2::default();
		let password_hash = argon2.hash_password(password, &salt).unwrap().hash.unwrap();

		let key = Key::from_slice(password_hash.as_bytes());
		let cipher = Aes256Gcm::new(key);

		let random_bytes = rand::thread_rng().gen::<[u8; 12]>();
		let nonce = Nonce::from_slice(&random_bytes);

		let encryptedreturn = EncryptedReturn {
			value: cipher.encrypt(nonce, value.as_ref()).expect("encryption failure!"),
			salt,
			nonce,
		};
		return encryptedreturn
	}
	// fn decrypt() {
	// 	let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
	// 		.expect("decryption failure!");
	// }
}