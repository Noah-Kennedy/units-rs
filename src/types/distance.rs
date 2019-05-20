use num_traits::{Num, FromPrimitive};

pub trait Distance {}

pub struct Meters<T>(T);

pub struct Centimeters<T>(T);

impl<T> From<Meters<T>> for Centimeters<T> where T: Num + FromPrimitive {
    fn from(meters: Meters<T>) -> Self {
        let Meters(meter_distance) = meters;
        Self(meter_distance * T::from_usize(100).unwrap())
    }
}

impl<T> From<Centimeters<T>> for Meters<T> where T: Num + FromPrimitive {
    fn from(meters: Meters<T>) -> Self {
        let Meters(meter_distance) = meters;
        Self(meter_distance / T::from_usize(100).unwrap())
    }
}

impl<T> Distance for Meters<T> {}

impl<T> Distance for Centimeters<T> {}