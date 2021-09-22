use crate::crypto::{EncryptedReturn, encrypt, decrypt};
use aes_gcm::{Key, Nonce};
use aes_gcm::Aes256Gcm;

mod crypto;


fn main() {
	let encrypted = encrypt(Vec::from("cum"), "cheese");

	let decrypted = decrypt(encrypted);
	println!("{}", String::from_utf8(decrypted).unwrap());

	let key = Key::from_slice(b"an example very very secret key.");
	let cipher = Aes256Gcm::new(key);

	let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

	let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
		.expect("encryption failure!"); // NOTE: handle this error to avoid panics!

	let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
		.expect("decryption failure!"); // NOTE: handle this error to avoid panics!
}