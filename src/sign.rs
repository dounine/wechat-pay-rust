use base64::Engine;
use base64::engine::general_purpose;
use rsa::{Pkcs1v15Sign, RsaPrivateKey};
use rsa::pkcs8::DecodePrivateKey;
use sha2::Digest;

/// sha256签名
/// ```Cargo.toml
/// base64 = "0.21.6"
/// rsa = { version = "0.9.6", features = ["sha2"] }
/// sha2 = "0.11.0-pre.0"
/// ```
pub fn sha256_sign<S>(private_key: S, content:S) -> String
    where S: AsRef<str>
{
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key.as_ref()).expect("failed to parse key");
    let hasher = sha2::Sha256::new().chain_update(content.as_ref()).finalize();
    let padding = Pkcs1v15Sign::new::<rsa::sha2::Sha256>();
    let sign_result = private_key.sign(padding, &hasher).expect("failed to sign");
    general_purpose::STANDARD.encode(sign_result)
}