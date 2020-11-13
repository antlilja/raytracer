pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

pub fn clamp01<T>(value: T) -> T
where
    T: PartialOrd + From<u8>,
{
    clamp(value, T::from(0 as u8), T::from(1 as u8))
}
