pub trait ConvertTo<T, E>: Sized {
    fn convert_to(self) -> Result<T, E>;
}

pub trait ConvertFrom<T>: Sized {
    fn convert_from(value: T) -> Self;
}