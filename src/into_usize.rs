use ordered_float::OrderedFloat;
use num_traits::AsPrimitive;

macro_rules! impl_into_usize_for_numeric_types {
    ($($t:ty),*) => {
        $(
            impl IntoUsize for $t {
                fn into_usize(&self) -> usize {
                    *self as usize
                }
            }
        )*
    };
}

pub trait IntoUsize {
    fn into_usize(&self) -> usize;
}

impl_into_usize_for_numeric_types! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    f32, f64
}

impl IntoUsize for OrderedFloat<f64> {
    fn into_usize(&self) -> usize {
        self.as_()
    }
}

impl IntoUsize for OrderedFloat<f32> {
    fn into_usize(&self) -> usize {
        self.as_()
    }
}
