use once_cell::sync::Lazy;
use chacha20poly1305::{ChaCha20Poly1305, KeyInit};
use chacha20poly1305::aead::generic_array::GenericArray;
use rand_core::OsRng;

pub static KEY: Lazy<GenericArray<u8, <ChaCha20Poly1305 as KeyInit>::KeySize>> = Lazy::new(|| {
    ChaCha20Poly1305::generate_key(&mut OsRng)
}); 