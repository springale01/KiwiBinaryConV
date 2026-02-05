use thiserror::Error;

use crate::converter::Base;

#[derive(Debug, Error)]
pub enum ConverterError {
    #[error("Failed to Convert")]
    FailedToConvert,
    #[error("number found not in base map")]
    NumberOutOfMap,
    #[error("number and base is no coherent")]
    NumberAndBaseNotCoherent,
    #[error("{0}")]
    CustomError(String),
    #[error("invalid character: {letter} restrained by the base {base}")]
    InvalidCharacter { letter: char, base: Base },
}
