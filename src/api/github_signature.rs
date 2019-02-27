use log::{info, warn};
use ring::{digest, hmac};

impl From<ring::error::Unspecified> for SignatureError {
    fn from(error: ring::error::Unspecified) -> Self {
        warn!("Got a bad signature: {:?}", error);
        SignatureError::BadSignature
    }
}

#[derive(Debug)]
pub enum SignatureError {
    BadSignature,
}

pub fn check_signature(secret: &str, signature: &str, body: &str) -> Result<(), SignatureError> {
    let v_key = hmac::VerificationKey::new(&digest::SHA1, secret.as_bytes());
    hmac::verify(&v_key, body.as_bytes(), signature.as_ref())?;
    info!("Good signature {} for GitHub", signature);
    Ok(())
}
