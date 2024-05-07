/// The conversion traits are based on the core From/Into traits, but are local to this crate, allowing these traits
/// to be use with foreign types

pub trait ConvertTo<T>: Sized {
    fn convert_to(self) -> T;
}

pub trait ConvertFrom<T>: Sized {
    fn convert_from(value: T) -> Self;
}

pub trait TryConvertTo<T>: Sized {
    type Error;

    fn try_convert_to(self) -> Result<T, Self::Error>;
}

pub trait TryConvertFrom<T>: Sized {
    type Error;

    fn try_convert_from(value: T) -> Result<Self, Self::Error>;
}


impl<T, U> ConvertTo<U> for T
    where
        U: ConvertFrom<T>,
{
    #[inline]
    fn convert_to(self) -> U {
        U::convert_from(self)
    }
}

impl<T, U> TryConvertTo<U> for T
    where
        U: TryConvertFrom<T>,
{
    type Error = U::Error;

    #[inline]
    fn try_convert_to(self) -> Result<U, U::Error> {
        U::try_convert_from(self)
    }
}
