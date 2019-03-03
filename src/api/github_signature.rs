use hex;
use log::{debug, warn};
use ring::{digest, hmac};

impl From<ring::error::Unspecified> for SignatureError {
    fn from(error: ring::error::Unspecified) -> Self {
        warn!("Got a bad request signature: {:?}", error);
        SignatureError::BadSignature
    }
}

impl From<hex::FromHexError> for SignatureError {
    fn from(error: hex::FromHexError) -> Self {
        warn!("Error decoding hex signature: {:?}", error);
        SignatureError::InvalidEncoding
    }
}

#[derive(Debug)]
pub enum SignatureError {
    BadSignature,
    InvalidFormat,
    InvalidEncoding,
}

pub fn check_signature(secret: &str, signature: &str, body: &str) -> Result<(), SignatureError> {
    let v_key = hmac::VerificationKey::new(&digest::SHA1, secret.as_bytes());
    let signature_parts = signature.split('=').collect::<Vec<&str>>();
    match signature_parts.len() {
        2 => {
            hmac::verify(&v_key, body.as_bytes(), &hex::decode(signature_parts[1])?)?;
            debug!("Good signature {} for GitHub", signature);
            Ok(())
        }
        _ => Err(SignatureError::InvalidFormat),
    }
}
