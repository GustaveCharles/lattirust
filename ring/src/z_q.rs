use ark_ff::{BigInt, BigInteger, Fp, Fp64, FpConfig, MontBackend, MontConfig, PrimeField};

use crate::{ConvertibleRing, SignedRepresentative, UnsignedRepresentative};

pub struct FqConfig<const Q: u64> {}

impl<const Q: u64> MontConfig<1> for FqConfig<Q> {
    const MODULUS: BigInt<1> = BigInt::<1>([Q]);
    // This works for primes of the form 2^x + 1
    const GENERATOR: Fp<MontBackend<Self, 1>, 1> = Fp::new(BigInt([3]));
    // This works for primes of the form 2^x + 1
    const TWO_ADIC_ROOT_OF_UNITY: Fp<MontBackend<Self, 1>, 1> = Fp::new(BigInt([3])); // TODO: implement this? Not using todo!() here to generate the docs
}

pub type Zq<const Q: u64> = Fp64<MontBackend<FqConfig<Q>, 1>>;

pub const fn const_fq_from<const Q: u64>(val: u64) -> Zq<Q> {
    Zq::new(BigInt::<1>([val]))
}

impl From<BigInt<1>> for UnsignedRepresentative {
    fn from(value: BigInt<1>) -> Self {
        UnsignedRepresentative(value.0[0] as u128)
    }
}

impl<C: FpConfig<1>> From<Fp64<C>> for UnsignedRepresentative {
    fn from(value: Fp64<C>) -> Self {
        UnsignedRepresentative::from(value.into_bigint())
    }
}

/// Map [0, q[ to [-m, m] using [0, m] -> [0, m] and ]m, q[ -> [-m, 0[, where m = (q-1)/2, assuming q is odd
impl<C: FpConfig<1>> From<Fp64<C>> for SignedRepresentative {
    fn from(value: Fp64<C>) -> Self {
        debug_assert!(Fp64::<C>::MODULUS.is_odd());
        let unsigned = UnsignedRepresentative::from(value).0 as i128;
        let v: BigInt<1> = value.into();
        let q_half: BigInt<1> = Fp64::<C>::MODULUS_MINUS_ONE_DIV_TWO;
        let q = UnsignedRepresentative::from(Fp64::<C>::MODULUS).0 as i128;
        if v > q_half {
            SignedRepresentative(unsigned - q)
        } else {
            SignedRepresentative(unsigned)
        }
    }
}

/// Map [-m, m] to [0, q[ using [0, m] -> [0, m] and [-m, 0[ -> [m, q[, where m = (q-1)/2, assuming q is odd
impl<C: FpConfig<1>> From<SignedRepresentative> for Fp64<C> {
    fn from(value: SignedRepresentative) -> Self {
        debug_assert!(Fp64::<C>::MODULUS.is_odd());
        let q: i128 = UnsignedRepresentative::from(Fp64::<C>::MODULUS).0 as i128;
        if value.0 < 0 {
            Fp64::<C>::from((value.0 + q) as u128)
        } else {
            Fp64::<C>::from(value.0 as u128)
        }
    }
}

impl<const Q: u64> ConvertibleRing for Zq<Q> {}
