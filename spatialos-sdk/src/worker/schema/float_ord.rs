//! Order floating point numbers, into this ordering:
//!
//!    NaN | -Infinity | x < 0 | -0 | +0 | x > 0 | +Infinity | NaN
//! Adapted from https://github.com/notriddle/rust-float-ord

use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::hash::{Hash, Hasher};
use core::mem::transmute;
use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A wrapper for floats, that implements total equality and ordering
/// and hashing.
#[derive(Clone, Copy, Debug)]
pub struct FloatOrd<T>(pub T);

macro_rules! float_ord_impl {
    ($f:ident, $i:ident, $n:expr) => {
        impl FloatOrd<$f> {
            fn convert(self) -> $i {
                let u = unsafe { transmute::<$f, $i>(self.0) };
                let bit = 1 << ($n - 1);
                if u & bit == 0 {
                    u | bit
                } else {
                    !u
                }
            }
        }

        impl PartialEq for FloatOrd<$f> {
            fn eq(&self, other: &Self) -> bool {
                self.convert() == other.convert()
            }
        }

        impl Eq for FloatOrd<$f> {}

        impl PartialOrd for FloatOrd<$f> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.convert().partial_cmp(&other.convert())
            }
        }

        impl Ord for FloatOrd<$f> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.convert().cmp(&other.convert())
            }
        }

        impl Hash for FloatOrd<$f> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.convert().hash(state);
            }
        }

        impl From<$f> for FloatOrd<$f> {
            fn from(source: $f) -> Self {
                FloatOrd(source)
            }
        }

        impl Deref for FloatOrd<$f> {
            type Target = $f;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for FloatOrd<$f> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Add for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn add(self, other: Self) -> Self {
                FloatOrd(self.0 + other.0)
            }
        }

        impl Add<$f> for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn add(self, other: $f) -> Self {
                FloatOrd(self.0 + other)
            }
        }

        impl Add<FloatOrd<$f>> for $f {
            type Output = FloatOrd<$f>;

            fn add(self, other: FloatOrd<$f>) -> FloatOrd<$f> {
                FloatOrd(self + other.0)
            }
        }

        impl AddAssign for FloatOrd<$f> {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl AddAssign<$f> for FloatOrd<$f> {
            fn add_assign(&mut self, other: $f) {
                self.0 += other;
            }
        }

        impl AddAssign<FloatOrd<$f>> for $f {
            fn add_assign(&mut self, other: FloatOrd<$f>) {
                *self += other.0;
            }
        }

        impl Div for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn div(self, other: Self) -> Self {
                FloatOrd(self.0 / other.0)
            }
        }

        impl Div<$f> for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn div(self, other: $f) -> Self {
                FloatOrd(self.0 / other)
            }
        }

        impl Div<FloatOrd<$f>> for $f {
            type Output = FloatOrd<$f>;

            fn div(self, other: FloatOrd<$f>) -> FloatOrd<$f> {
                FloatOrd(self / other.0)
            }
        }

        impl DivAssign for FloatOrd<$f> {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl DivAssign<$f> for FloatOrd<$f> {
            fn div_assign(&mut self, other: $f) {
                self.0 /= other;
            }
        }

        impl DivAssign<FloatOrd<$f>> for $f {
            fn div_assign(&mut self, other: FloatOrd<$f>) {
                *self /= other.0;
            }
        }

        impl Mul for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn mul(self, rhs: Self) -> Self {
                FloatOrd(self.0 * rhs.0)
            }
        }

        impl Mul<$f> for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn mul(self, rhs: $f) -> Self {
                FloatOrd(self.0 * rhs)
            }
        }

        impl Mul<FloatOrd<$f>> for $f {
            type Output = FloatOrd<$f>;

            fn mul(self, rhs: FloatOrd<$f>) -> FloatOrd<$f> {
                FloatOrd(self * rhs.0)
            }
        }

        impl MulAssign for FloatOrd<$f> {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl MulAssign<$f> for FloatOrd<$f> {
            fn mul_assign(&mut self, other: $f) {
                self.0 *= other;
            }
        }

        impl MulAssign<FloatOrd<$f>> for $f {
            fn mul_assign(&mut self, other: FloatOrd<$f>) {
                *self *= other.0;
            }
        }

        impl Sub for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn sub(self, other: Self) -> Self {
                FloatOrd(self.0 - other.0)
            }
        }

        impl Sub<$f> for FloatOrd<$f> {
            type Output = FloatOrd<$f>;

            fn sub(self, other: $f) -> Self {
                FloatOrd(self.0 - other)
            }
        }

        impl Sub<FloatOrd<$f>> for $f {
            type Output = FloatOrd<$f>;

            fn sub(self, other: FloatOrd<$f>) -> FloatOrd<$f> {
                FloatOrd(self - other.0)
            }
        }

        impl SubAssign for FloatOrd<$f> {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl SubAssign<$f> for FloatOrd<$f> {
            fn sub_assign(&mut self, other: $f) {
                self.0 -= other;
            }
        }

        impl SubAssign<FloatOrd<$f>> for $f {
            fn sub_assign(&mut self, other: FloatOrd<$f>) {
                *self -= other.0;
            }
        }
    }
}

float_ord_impl!(f32, u32, 32);
float_ord_impl!(f64, u64, 64);

#[cfg(test)]
mod tests {

    use rand::{Rng, thread_rng};
    use std::iter;
    use std::prelude::v1::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use super::FloatOrd;

    #[test]
    fn test_ord() {
        assert!(FloatOrd(1.0f64) < FloatOrd(2.0f64));
        assert!(FloatOrd(2.0f32) > FloatOrd(1.0f32));
        assert!(FloatOrd(1.0f64) == FloatOrd(1.0f64));
        assert!(FloatOrd(1.0f32) == FloatOrd(1.0f32));
        assert!(FloatOrd(0.0f64) > FloatOrd(-0.0f64));
        assert!(FloatOrd(0.0f32) > FloatOrd(-0.0f32));
        assert!(FloatOrd(::core::f64::NAN) == FloatOrd(::core::f64::NAN));
        assert!(FloatOrd(::core::f32::NAN) == FloatOrd(::core::f32::NAN));
        assert!(FloatOrd(-::core::f64::NAN) < FloatOrd(::core::f64::NAN));
        assert!(FloatOrd(-::core::f32::NAN) < FloatOrd(::core::f32::NAN));
        assert!(FloatOrd(-::core::f64::INFINITY) < FloatOrd(::core::f64::INFINITY));
        assert!(FloatOrd(-::core::f32::INFINITY) < FloatOrd(::core::f32::INFINITY));
        assert!(FloatOrd(::core::f64::INFINITY) < FloatOrd(::core::f64::NAN));
        assert!(FloatOrd(::core::f32::INFINITY) < FloatOrd(::core::f32::NAN));
        assert!(FloatOrd(-::core::f64::NAN) < FloatOrd(::core::f64::INFINITY));
        assert!(FloatOrd(-::core::f32::NAN) < FloatOrd(::core::f32::INFINITY));
    }

    #[test]
    fn test_ord_numbers() {
        let mut rng = thread_rng();
        for n in 0..16 {
            for l in 0..16 {
                let v = iter::repeat(()).map(|()| rng.gen())
                    .map(|x: f64| x % (1 << l) as i64 as f64)
                    .take((1 << n))
                    .collect::<Vec<_>>();
                assert!(v.windows(2).all(|w| (w[0] <= w[1]) == (FloatOrd(w[0]) <= FloatOrd(w[1]))));
            }
        }
    }

    fn hash<F: Hash>(f: F) -> u64 {
        let mut hasher = DefaultHasher::new();
        f.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_hash() {
        assert_ne!(hash(FloatOrd(0.0f64)), hash(FloatOrd(-0.0f64)));
        assert_ne!(hash(FloatOrd(0.0f32)), hash(FloatOrd(-0.0f32)));
        assert_eq!(hash(FloatOrd(-0.0f64)), hash(FloatOrd(-0.0f64)));
        assert_eq!(hash(FloatOrd(0.0f32)), hash(FloatOrd(0.0f32)));
        assert_ne!(hash(FloatOrd(::core::f64::NAN)), hash(FloatOrd(-::core::f64::NAN)));
        assert_ne!(hash(FloatOrd(::core::f32::NAN)), hash(FloatOrd(-::core::f32::NAN)));
        assert_eq!(hash(FloatOrd(::core::f64::NAN)), hash(FloatOrd(::core::f64::NAN)));
        assert_eq!(hash(FloatOrd(-::core::f32::NAN)), hash(FloatOrd(-::core::f32::NAN)));
    }
}