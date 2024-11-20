use ark_ff::{BitIteratorBE, BitIteratorLE, Field, Fp, FpConfig};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    One, UniformRand, Zero,
};

use crate::traits::FromRandomBytes;

pub trait Ring: 'static +
Copy +
Clone +
Debug +
Display +
Default +
Send +
Sync +
Eq +
Zero +
One +
Neg<Output=Self> +
UniformRand +
Sized +
Hash +
CanonicalSerialize +
CanonicalDeserialize +
Add<Self, Output=Self> +
Sub<Self, Output=Self> +
Mul<Self, Output=Self> +
AddAssign<Self> +
SubAssign<Self> +
MulAssign<Self> +
    for<'a> Add<&'a Self, Output=Self> +
    for<'a> Sub<&'a Self, Output=Self> +
    for<'a> Mul<&'a Self, Output=Self> +
    for<'a> AddAssign<&'a Self> +
    for<'a> SubAssign<&'a Self> +
    for<'a> MulAssign<&'a Self> +
    for<'a> Add<&'a mut Self, Output=Self> +
    for<'a> Sub<&'a mut Self, Output=Self> +
    for<'a> Mul<&'a mut Self, Output=Self> +
    for<'a> AddAssign<&'a mut Self> +
    for<'a> SubAssign<&'a mut Self> +
    for<'a> MulAssign<&'a mut Self> +
Sum<Self> +
    for<'a> Sum<&'a Self> +
Product<Self> +
    for<'a> Product<&'a Self> +
Sum<Self> +
From<u128> +
From<u64> +
From<u32> +
From<u16> +
From<u8> +
From<bool> +
// Differs from arkworks
FromRandomBytes<Self> +
CanonicalSerialize +
CanonicalDeserialize
{
    /// The additive identity of the ring.
    const ZERO: Self;
    /// The multiplicative identity of the ring.
    const ONE: Self;

    /// Returns `sum([a_i * b_i])`.
    #[inline]
    fn sum_of_products<const T: usize>(a: &[Self; T], b: &[Self; T]) -> Self {
        let mut sum = Self::zero();
        for i in 0..a.len() {
            sum += a[i] * b[i];
        }
        sum
    }

    fn square_in_place(&mut self) -> &mut Self {
        *self *= *self;
        self
    }

    /// Returns `self^exp`, where `exp` is an integer represented with `u64` limbs,
    /// least significant limb first.
    #[must_use]
    fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        let mut res = Self::one();

        for i in BitIteratorBE::without_leading_zeros(exp) {
            res.square_in_place();

            if i {
                res *= self;
            }
        }
        res
    }

    /// Exponentiates a field element `f` by a number represented with `u64`
    /// limbs, using a precomputed table containing as many powers of 2 of
    /// `f` as the 1 + the floor of log2 of the exponent `exp`, starting
    /// from the 1st power. That is, `powers_of_2` should equal `&[p, p^2,
    /// p^4, ..., p^(2^n)]` when `exp` has at most `n` bits.
    ///
    /// This returns `None` when a power is missing from the table.
    #[inline]
    fn pow_with_table<S: AsRef<[u64]>>(powers_of_2: &[Self], exp: S) -> Option<Self> {
        let mut res = Self::one();
        for (pow, bit) in BitIteratorLE::without_trailing_zeros(exp).enumerate() {
            if bit {
                res *= powers_of_2.get(pow)?;
            }
        }
        Some(res)
    }
}

impl<C: FpConfig<N>, const N: usize> FromRandomBytes<Fp<C, N>> for Fp<C, N> {
    fn byte_size() -> usize {
        Self::zero().uncompressed_size() + 9 // TODO: check if this is correct; this is inferred from Fp<C, N>::from_random_bytes()
    }

    fn try_from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_random_bytes(bytes)
    }
}

impl<C: FpConfig<N>, const N: usize> Ring for Fp<C, N> {
    const ZERO: Self = <Fp<C, N> as Field>::ZERO;

    const ONE: Self = <Fp<C, N> as Field>::ONE;
}
