use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::ChaCha20Poly1305;

pub fn encrypt(cleartext: &str, key: &[u8]) -> Vec<u8> {

    let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let mut obsf = cipher.encrypt(&nonce, cleartext.as_bytes()).unwrap();

    obsf.splice(..0, nonce.iter().copied());
    
    obsf
}
