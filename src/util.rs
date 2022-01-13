use num_traits::{One, Zero};
use std::ops::{Add, Div, Rem, Sub};

pub fn round_up_to<T: Zero + Add<Output = T> + Rem<Output = T> + Sub<Output = T> + Copy>(
    val: T,
    div: T,
) -> T {
    if (val % div).is_zero() {
        val
    } else {
        val + div - val % div
    }
}

pub fn div_round_up<T: One + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Copy>(
    val: T,
    div: T,
) -> T {
    (val + (div - T::one())) / div
}
