use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoPalsError {
    #[error("Cannot decode Hex string, is the string total characters divisible by two?")]
    CannotDecodeHex(String),
    #[error("The two buffers are different in size, cannot xor them")]
    DifferentSizedBuffers,
}
