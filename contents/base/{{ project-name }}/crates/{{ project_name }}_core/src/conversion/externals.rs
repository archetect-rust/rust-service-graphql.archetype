use std::str::FromStr;
use async_graphql::ID;
use uuid::Uuid;
use crate::errors::TypeConversionError;
use crate::{ConvertFrom, TryConvertFrom};

impl TryConvertFrom<ID> for Uuid {
    type Error = TypeConversionError;

    fn try_convert_from(value: ID) -> Result<Self, Self::Error> {
        Uuid::from_str(value.0.as_str())
            .map_err(|_err| TypeConversionError::InvalidUuid { original: value.0 })
    }
}

impl ConvertFrom<Uuid> for ID {
    fn convert_from(value: Uuid) -> Self {
        ID::from(value.to_string())
    }
}