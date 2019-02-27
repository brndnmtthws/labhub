use ring::{digest, hmac, rand};

#[derive(Debug)]
pub enum SignatureError {
    BadSignature,
}

fn check_signature() -> Result<(), SignatureError> {
    let v_key = hmac::VerificationKey::new(&digest::SHA384, key_value.as_ref());
let mut msg = Vec::<u8>::new();
for part in &parts {
    msg.extend(part.as_bytes());
}
hmac::verify(&v_key, &msg.as_ref(), signature.as_ref())?;
}
