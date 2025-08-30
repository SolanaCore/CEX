use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}, PasswordHash, PasswordVerifier};


pub static ARGON2: Lazy<Argon2> = Lazy::new(|| {
    Argon2::default()
})

impl Argon2 {
pub fn get_argon2() -> &'static Argon2 {
    *ARGON2
}
}

pub fn get_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}
