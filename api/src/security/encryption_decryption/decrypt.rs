use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::ChaCha20Poly1305;

pub fn decrypt(obsf: &[u8], key: &[u8]) -> String {
    type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;

    let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));

    let (nonce, ciphertext) = obsf.split_at(NonceSize::to_usize());

    let nonce = GenericArray::from_slice(nonce);

    let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();

    String::from_utf8(plaintext).unwrap()
}

