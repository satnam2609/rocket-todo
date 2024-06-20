use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::Error;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::model::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn authorize_password(user: &User, credentials: Credentials) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let db_hash = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;

    let session_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    Ok(session_id)
}

pub fn generate_hash(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(hashed_password.to_string())
}
