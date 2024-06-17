use crate::path::scalar::Scalar;
pub(crate) const FLOAT_PI: f32 = 3.14159265;

const MAX_I32_FITS_IN_F32: f32 = 2147483520.0;
const MIN_I32_FITS_IN_F32: f32 = -MAX_I32_FITS_IN_F32;

// TODO: is there an std alternative?
/// Custom float to integer conversion routines.
pub trait SaturateCast<T>: Sized {
    /// Return the closest integer for the given float.
    fn saturate_from(n: T) -> Self;
}

impl SaturateCast<f32> for i32 {
    /// Return the closest integer for the given float.
    ///
    /// Returns MAX_I32_FITS_IN_F32 for NaN.
    fn saturate_from(mut x: f32) -> Self {
        x = if x < MAX_I32_FITS_IN_F32 {
            x
        } else {
            MAX_I32_FITS_IN_F32
        };
        x = if x > MIN_I32_FITS_IN_F32 {
            x
        } else {
            MIN_I32_FITS_IN_F32
        };
        x as i32
    }
}

impl SaturateCast<f64> for i32 {
    /// Return the closest integer for the given double.
    ///
    /// Returns i32::MAX for NaN.
    fn saturate_from(mut x: f64) -> Self {
        x = if x < i32::MAX as f64 {
            x
        } else {
            i32::MAX as f64
        };
        x = if x > i32::MIN as f64 {
            x
        } else {
            i32::MIN as f64
        };
        x as i32
    }
}

/// Custom float to integer rounding routines.
#[allow(missing_docs)]
pub trait SaturateRound<T>: SaturateCast<T> {
    fn saturate_floor(n: T) -> Self;
    fn saturate_ceil(n: T) -> Self;
    fn saturate_round(n: T) -> Self;
}

impl SaturateRound<f32> for i32 {
    fn saturate_floor(x: f32) -> Self {
        Self::saturate_from(x.floor())
    }

    fn saturate_ceil(x: f32) -> Self {
        Self::saturate_from(x.ceil())
    }

    fn saturate_round(x: f32) -> Self {
        Self::saturate_from(x.floor() + 0.5)
    }
}

/// Return the float as a 2s compliment int. Just to be used to compare floats
/// to each other or against positive float-bit-constants (like 0). This does
/// not return the int equivalent of the float, just something cheaper for
/// compares-only.
pub(crate) fn f32_as_2s_compliment(x: f32) -> i32 {
    sign_bit_to_2s_compliment(x.to_bits() as i32)
}

/// Convert a sign-bit int (i.e. float interpreted as int) into a 2s compliement
/// int. This also converts -0 (0x80000000) to 0. Doing this to a float allows
/// it to be compared using normal C operators (<, <=, etc.)
fn sign_bit_to_2s_compliment(mut x: i32) -> i32 {
    if x < 0 {
        x &= 0x7FFFFFFF;
        x = -x;
    }

    x
}

/// An immutable `f32` that is larger than 0 but less then 1.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
#[repr(transparent)]
pub struct NormalizedF32Exclusive(FiniteF32);

impl NormalizedF32Exclusive {
    /// Just a random, valid number.
    pub const ANY: Self = Self::HALF;

    /// A predefined 0.5 value.
    pub const HALF: Self = NormalizedF32Exclusive(unsafe { FiniteF32::new_unchecked(0.5) });

    /// Creates a `NormalizedF32Exclusive`.
    pub fn new(n: f32) -> Option<Self> {
        if n > 0.0 && n < 1.0 {
            // `n` is guarantee to be finite after the bounds check.
            FiniteF32::new(n).map(NormalizedF32Exclusive)
        } else {
            None
        }
    }

    /// Creates a `NormalizedF32Exclusive` clamping the given value.
    ///
    /// Returns zero in case of NaN or infinity.
    pub fn new_bounded(n: f32) -> Self {
        let n = n.bound(f32::EPSILON, 1.0 - f32::EPSILON);
        // `n` is guarantee to be finite after clamping.
        debug_assert!(n.is_finite());
        NormalizedF32Exclusive(unsafe { FiniteF32::new_unchecked(n) })
    }

    /// Returns the value as a primitive type.
    pub fn get(self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF32`.
    pub fn to_normalized(self) -> NormalizedF32 {
        // NormalizedF32 is (0,1), while NormalizedF32 is [0,1], so it will always fit.
        unsafe { NormalizedF32::new_unchecked(self.0.get()) }
    }
}

/// An immutable, finite [`f32`].
///
/// Unlike [`f32`], implements [`Eq`], [`Ord`] and [`Hash`].
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct FiniteF32(f32);

impl FiniteF32 {
    /// Creates a finite [`f32`].
    ///
    /// Returns [`None`] for `NaN` and infinity.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() {
            Some(FiniteF32(n))
        } else {
            None
        }
    }

    /// Creates a finite [`f32`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        FiniteF32(n)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl Eq for FiniteF32 {}

impl PartialEq for FiniteF32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for FiniteF32 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.0 < other.0 {
            core::cmp::Ordering::Less
        } else if self.0 > other.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for FiniteF32 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::hash::Hash for FiniteF32 {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq<f32> for FiniteF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f64`].
///
/// Unlike [`f64`], implements [`Eq`], [`Ord`] and [`Hash`].
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a finite [`f64`].
    ///
    /// Returns [`None`] for `NaN` and infinity.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n.is_finite() {
            Some(FiniteF64(n))
        } else {
            None
        }
    }

    /// Creates a finite [`f64`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        FiniteF64(n)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f64 {
        self.0
    }
}

impl Eq for FiniteF64 {}

impl PartialEq for FiniteF64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for FiniteF64 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.0 < other.0 {
            core::cmp::Ordering::Less
        } else if self.0 > other.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for FiniteF64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::hash::Hash for FiniteF64 {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq<f64> for FiniteF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f32`] that is known to be >= 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
#[repr(transparent)]
pub struct PositiveF32(FiniteF32);

impl PositiveF32 {
    /// A [`PositiveF32`] value initialized with zero.
    pub const ZERO: Self = PositiveF32(FiniteF32(0.0));

    /// Creates a new [`PositiveF32`] if the given value is >= 0.
    ///
    /// Returns [`None`] for negative, `NaN` and infinity.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() && n >= 0.0 {
            Some(PositiveF32(FiniteF32(n)))
        } else {
            None
        }
    }

    /// Creates a new [`PositiveF32`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and >= 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        PositiveF32(FiniteF32(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a [`FiniteF32`].
    #[inline]
    pub const fn get_finite(&self) -> FiniteF32 {
        self.0
    }
}

impl PartialEq<f32> for PositiveF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f64`] that is known to be >= 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
#[repr(transparent)]
pub struct PositiveF64(FiniteF64);

impl PositiveF64 {
    /// A [`PositiveF64`] value initialized with zero.
    pub const ZERO: Self = PositiveF64(FiniteF64(0.0));

    /// Creates a new [`PositiveF64`] if the given value is >= 0.
    ///
    /// Returns [`None`] for negative, `NaN` and infinity.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n.is_finite() && n >= 0.0 {
            Some(PositiveF64(FiniteF64(n)))
        } else {
            None
        }
    }

    /// Creates a new [`PositiveF64`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and >= 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        PositiveF64(FiniteF64(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f64 {
        self.0.get()
    }

    /// Returns the value as a [`FiniteF64`].
    #[inline]
    pub const fn get_finite(&self) -> FiniteF64 {
        self.0
    }
}

impl PartialEq<f64> for PositiveF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f32`] that is known to be > 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NonZeroPositiveF32(FiniteF32);

impl NonZeroPositiveF32 {
    /// Creates a new [`NonZeroPositiveF32`] if the given value is > 0.
    ///
    /// Returns [`None`] for negative, zero, `NaN` and infinity.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() && n > 0.0 {
            Some(NonZeroPositiveF32(FiniteF32(n)))
        } else {
            None
        }
    }

    /// Creates a new [`NonZeroPositiveF32`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and > 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        NonZeroPositiveF32(FiniteF32(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a [`FiniteF32`].
    #[inline]
    pub const fn get_finite(&self) -> FiniteF32 {
        self.0
    }
}

impl PartialEq<f32> for NonZeroPositiveF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f64`] that is known to be > 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NonZeroPositiveF64(FiniteF64);

impl NonZeroPositiveF64 {
    /// Creates a new [`NonZeroPositiveF64`] if the given value is > 0.
    ///
    /// Returns [`None`] for negative, zero, NaN and infinity.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n.is_finite() && n > 0.0 {
            Some(NonZeroPositiveF64(FiniteF64(n)))
        } else {
            None
        }
    }

    /// Creates a new [`NonZeroPositiveF64`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and > 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        NonZeroPositiveF64(FiniteF64(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f64 {
        self.0.get()
    }

    /// Returns the value as a [`FiniteF64`].
    #[inline]
    pub const fn get_finite(&self) -> FiniteF64 {
        self.0
    }
}

impl PartialEq<f64> for NonZeroPositiveF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f32`] in a 0..=1 range.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NormalizedF32(FiniteF32);

impl NormalizedF32 {
    /// A [`NormalizedF32`] value initialized with zero.
    pub const ZERO: Self = NormalizedF32(FiniteF32(0.0));
    /// A [`NormalizedF32`] value initialized with one.
    pub const ONE: Self = NormalizedF32(FiniteF32(1.0));

    /// Creates a [`NormalizedF32`] if the given value is in a 0..=1 range.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() && (0.0..=1.0).contains(&n) {
            Some(NormalizedF32(FiniteF32(n)))
        } else {
            None
        }
    }

    /// Creates a new [`NormalizedF32`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be in 0..=1 range.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        NormalizedF32(FiniteF32(n))
    }

    /// Creates a [`NormalizedF32`] clamping the given value to a 0..=1 range.
    ///
    /// Returns zero in case of `NaN` or infinity.
    #[inline]
    pub fn new_clamped(n: f32) -> Self {
        if n.is_finite() {
            NormalizedF32(FiniteF32(clamp_f32(0.0, n, 1.0)))
        } else {
            Self::ZERO
        }
    }

    /// Creates a [`NormalizedF32`] by dividing the given value by 255.
    #[inline]
    pub fn new_u8(n: u8) -> Self {
        NormalizedF32(FiniteF32(f32::from(n) / 255.0))
    }

    /// Creates a [`NormalizedF64`] by dividing the given value by 65535.
    #[inline]
    pub fn new_u16(n: u16) -> Self {
        NormalizedF32(FiniteF32(f32::from(n) / 65535.0))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a [`FiniteF32`].
    #[inline]
    pub const fn get_finite(&self) -> FiniteF32 {
        self.0
    }

    /// Returns the value as a [`u8`].
    #[inline]
    pub fn to_u8(&self) -> u8 {
        ((self.0).0 * 255.0 + 0.5) as u8
    }

    /// Returns the value as a [`u16`].
    #[inline]
    pub fn to_u16(&self) -> u16 {
        ((self.0).0 * 65535.0 + 0.5) as u16
    }
}

impl core::ops::Mul<NormalizedF32> for NormalizedF32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_clamped((self.0).0 * (rhs.0).0)
    }
}

impl PartialEq<f32> for NormalizedF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

/// An immutable, finite [`f64`] in a 0..=1 range.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NormalizedF64(FiniteF64);

impl NormalizedF64 {
    /// A [`NormalizedF64`] value initialized with zero.
    pub const ZERO: Self = NormalizedF64(FiniteF64(0.0));
    /// A [`NormalizedF64`] value initialized with one.
    pub const ONE: Self = NormalizedF64(FiniteF64(1.0));

    /// Creates a [`NormalizedF64`] if the given value is in a 0..=1 range.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if (0.0..=1.0).contains(&n) {
            Some(NormalizedF64(FiniteF64(n)))
        } else {
            None
        }
    }

    /// Creates a new [`NormalizedF64`] without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be in 0..=1 range.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        NormalizedF64(FiniteF64(n))
    }

    /// Creates a [`NormalizedF64`] clamping the given value to a 0..=1 range.
    ///
    /// Returns zero in case of `NaN` or infinity.
    #[inline]
    pub fn new_clamped(n: f64) -> Self {
        if n.is_finite() {
            NormalizedF64(FiniteF64(clamp_f64(0.0, n, 1.0)))
        } else {
            Self::ZERO
        }
    }

    /// Creates a [`NormalizedF64`] by dividing the given value by 255.
    #[inline]
    pub fn new_u8(n: u8) -> Self {
        NormalizedF64(FiniteF64(f64::from(n) / 255.0))
    }

    /// Creates a [`NormalizedF64`] by dividing the given value by 65535.
    #[inline]
    pub fn new_u16(n: u16) -> Self {
        NormalizedF64(FiniteF64(f64::from(n) / 65535.0))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(self) -> f64 {
        self.0.get()
    }

    /// Returns the value as a [`FiniteF64`].
    #[inline]
    pub const fn get_finite(&self) -> FiniteF64 {
        self.0
    }

    /// Returns the value as a [`u8`].
    #[inline]
    pub fn to_u8(&self) -> u8 {
        ((self.0).0 * 255.0 + 0.5) as u8
    }

    /// Returns the value as a [`u16`].
    #[inline]
    pub fn to_u16(&self) -> u16 {
        ((self.0).0 * 65535.0 + 0.5) as u16
    }
}

impl core::ops::Mul<NormalizedF64> for NormalizedF64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_clamped((self.0).0 * (rhs.0).0)
    }
}

impl PartialEq<f64> for NormalizedF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

#[inline]
fn clamp_f32(min: f32, val: f32, max: f32) -> f32 {
    max.min(val).max(min)
}

#[inline]
fn clamp_f64(min: f64, val: f64, max: f64) -> f64 {
    max.min(val).max(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_f32() {
        assert_eq!(FiniteF32::new(0.0).map(|n| n.get()), Some(0.0));
        assert_eq!(FiniteF32::new(core::f32::NAN), None);
        assert_eq!(FiniteF32::new(core::f32::INFINITY), None);
        assert_eq!(FiniteF32::new(core::f32::NEG_INFINITY), None);
    }

    #[test]
    fn positive_f32() {
        assert_eq!(NonZeroPositiveF32::new(-1.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF32::new(0.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF32::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(
            NonZeroPositiveF32::new(core::f32::EPSILON).map(|n| n.get()),
            Some(core::f32::EPSILON)
        );
        assert_eq!(
            NonZeroPositiveF32::new(-core::f32::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NonZeroPositiveF32::new(core::f32::NAN), None);
        assert_eq!(NonZeroPositiveF32::new(core::f32::INFINITY), None);
        assert_eq!(NonZeroPositiveF32::new(core::f32::NEG_INFINITY), None);
    }

    #[test]
    fn positive_f64() {
        assert_eq!(NonZeroPositiveF32::new(-1.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF64::new(0.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF64::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(
            NonZeroPositiveF64::new(core::f64::EPSILON).map(|n| n.get()),
            Some(core::f64::EPSILON)
        );
        assert_eq!(
            NonZeroPositiveF64::new(-core::f64::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NonZeroPositiveF64::new(core::f64::NAN), None);
        assert_eq!(NonZeroPositiveF64::new(core::f64::INFINITY), None);
        assert_eq!(NonZeroPositiveF64::new(core::f64::NEG_INFINITY), None);
    }

    #[test]
    fn norm_f32() {
        assert_eq!(NormalizedF32::new(-0.5), None);
        assert_eq!(
            NormalizedF32::new(-core::f32::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NormalizedF32::new(0.0).map(|n| n.get()), Some(0.0));
        assert_eq!(NormalizedF32::new(0.5).map(|n| n.get()), Some(0.5));
        assert_eq!(NormalizedF32::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(NormalizedF32::new(1.5), None);
        assert_eq!(NormalizedF32::new(core::f32::NAN), None);
        assert_eq!(NormalizedF32::new(core::f32::INFINITY), None);
        assert_eq!(NormalizedF32::new(core::f32::NEG_INFINITY), None);
    }

    #[test]
    fn clamped_norm_f32() {
        assert_eq!(NormalizedF32::new_clamped(-0.5).get(), 0.0);
        assert_eq!(NormalizedF32::new_clamped(0.5).get(), 0.5);
        assert_eq!(NormalizedF32::new_clamped(1.5).get(), 1.0);
        assert_eq!(NormalizedF32::new_clamped(core::f32::NAN).get(), 0.0);
        assert_eq!(NormalizedF32::new_clamped(core::f32::INFINITY).get(), 0.0);
        assert_eq!(
            NormalizedF32::new_clamped(core::f32::NEG_INFINITY).get(),
            0.0
        );
    }

    #[test]
    fn norm_f64() {
        assert_eq!(NormalizedF64::new(-0.5), None);
        assert_eq!(
            NormalizedF64::new(-core::f64::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NormalizedF64::new(0.0).map(|n| n.get()), Some(0.0));
        assert_eq!(NormalizedF64::new(0.5).map(|n| n.get()), Some(0.5));
        assert_eq!(NormalizedF64::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(NormalizedF64::new(1.5), None);
        assert_eq!(NormalizedF64::new(core::f64::NAN), None);
        assert_eq!(NormalizedF64::new(core::f64::INFINITY), None);
        assert_eq!(NormalizedF64::new(core::f64::NEG_INFINITY), None);
    }

    #[test]
    fn clamped_norm_f64() {
        assert_eq!(NormalizedF64::new_clamped(-0.5).get(), 0.0);
        assert_eq!(NormalizedF64::new_clamped(0.5).get(), 0.5);
        assert_eq!(NormalizedF64::new_clamped(1.5).get(), 1.0);
        assert_eq!(NormalizedF64::new_clamped(core::f64::NAN).get(), 0.0);
        assert_eq!(NormalizedF64::new_clamped(core::f64::INFINITY).get(), 0.0);
        assert_eq!(
            NormalizedF64::new_clamped(core::f64::NEG_INFINITY).get(),
            0.0
        );
    }
}
