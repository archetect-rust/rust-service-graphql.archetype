#[derive(Debug, thiserror::Error)]
pub enum TypeConversionError {
    #[error("{original} is not a valid Uuid")]
    InvalidUuid {
        original: String
    },
}