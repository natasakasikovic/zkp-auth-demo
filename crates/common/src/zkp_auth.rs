use sha2::{Digest, Sha256};
use zkp_auth_schnorr::{AuthProofBundle, PublicKey, SecretKey};

pub const ZKP_AUTH_HEADER: &str = "x-zkp-auth-proof";
pub const ORDER_SERVICE_ID: &str = "order-service";

pub fn secret_key_from_material(secret_material: &str) -> SecretKey {
    let digest: [u8; 32] = Sha256::digest(secret_material.as_bytes()).into();
    SecretKey::from_bytes(digest)
}

pub fn encode_auth_proof(bundle: &AuthProofBundle) -> Result<String, serde_json::Error> {
    serde_json::to_string(bundle)
}

pub fn decode_auth_proof(header_value: &str) -> Result<AuthProofBundle, serde_json::Error> {
    serde_json::from_str(header_value)
}

pub fn default_order_secret_key() -> SecretKey {
    secret_key_from_material("dev-only-order-service-secret")
}

pub fn default_order_public_key() -> PublicKey {
    default_order_secret_key().public_key()
}
