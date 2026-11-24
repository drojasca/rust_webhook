use rand::{distr::Alphanumeric, RngExt};
use core::panic;
use std::error::Error;
use sha2::{Digest, Sha256};

fn generate_hex() -> Result<String, Box<dyn Error>> {
    const PASSWORD_LEN: usize = 16;
    let mut rng = rand::rng();

    let chars: String = (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    Ok(hex::encode(chars))
}

fn generate_api_key(permission: &str) -> Result<String, Box<dyn Error>>{
    const PASSWORD_LEN: usize = 16;
    let mut rng = rand::rng();

    let chars: String = (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    let hex = hex::encode(chars);
    let prefix = match permission {
        "admin" => "admin_",
        "subscriber" => "sub_",
        "publisher" => "pub_",
        &_ => panic!("unrecognized permission type: {permission}")
    }

    let key = format!("{prefix}{hex}");

    Ok(key)
}

fn hash_key(api_key: String) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::default();
    hasher.update(api_key.as_bytes());
    let hex_hash = hex::encode(hasher.finalize());
    Ok(hex_hash)
}
