use sha2::{Digest, Sha256};

pub fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed = hasher.finalize();

    hex::encode(hashed)
}