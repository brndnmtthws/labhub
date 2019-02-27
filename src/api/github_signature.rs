use crate::config;
use ring::{digest, hmac, rand};

#[derive(Debug)]
pub enum SignatureError {
    BadSignature,
}

fn check_signature(body: &String) -> Result<(), SignatureError> {
    let v_key = hmac::VerificationKey::new(&digest::SHA1, config::GITHUB_WEBHOOK_SECRET.as_ref());
    hmac::verify(&v_key, body.as_bytes(), signature.as_ref())?;
}
