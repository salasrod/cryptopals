use openssl::error::ErrorStack;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoPalsError {
    #[error("Cannot decode Hex string, is the string total characters divisible by two?")]
    CannotDecodeHex(String),
    #[error("The two buffers are different in size, cannot xor them")]
    DifferentSizedBuffers,
    #[error("An error ocurred when decoding Base64 string")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Range is outside the bounds of the slice")]
    RangeOutsideBounds,
    #[error("Error in the OpenSSL stack")]
    OpenSSLStackError(#[from] ErrorStack),
}
