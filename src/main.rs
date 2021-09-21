mod crypto;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use rand::Rng;
use aes_gcm::aes::cipher::generic_array::sequence::GenericSequence;

fn main() {




}