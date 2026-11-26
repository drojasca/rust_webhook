use rand::{distr::Alphanumeric, RngExt};
use sha2::{Digest, Sha256};

pub(crate) enum Permission {
    Admin,
    Subscriber,
    Publisher,
}

pub(crate) fn generate_api_key(permission: Permission) -> String {
    const PASSWORD_LEN: usize = 16;
    let mut rng = rand::rng();

    let chars: String = (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    let hex = hex::encode(chars);
    let prefix = match permission {
        Permission::Admin => "admin_",
        Permission::Subscriber => "sub_",
        Permission::Publisher => "pub_",
    };

    format!("{prefix}{hex}")
}

pub(crate) fn hash_key(api_key: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(api_key.as_bytes());
    hex::encode(hasher.finalize())
}
