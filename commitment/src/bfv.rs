#![allow(non_snake_case)]

use lattirust_arithmetic::challenge_set::weighted_ternary::WeightedTernaryChallengeSet;
use lattirust_arithmetic::linear_algebra::{Matrix, Vector};
use lattirust_arithmetic::ntt::ntt_modulus;

// use fhe::bfv::Ciphertext;
use lattirust_arithmetic::ring::{ConvertibleRing, PolyRing, Pow2CyclotomicPolyRing, Pow2CyclotomicPolyRingNTT, SignedRepresentative, UnsignedRepresentative, Zq};
use rand::{CryptoRng, RngCore};
use rand_distr::Normal;
use rand::distributions::{Distribution};
use ark_ff::{One, UniformRand, Zero};
use ark_std::rand::prelude::SliceRandom;
use ark_std::rand;
use lattirust_arithmetic::challenge_set::ternary;
use lattirust_arithmetic::traits::FromRandomBytes;

// TODO: toy sampling, need to use OpenFHE code 
pub fn get_gaussian_vec<
    T: RngCore + CryptoRng, 
    const Q: u64
>(  std_dev: f64, 
    dimension: usize, 
    rng: &mut T
) -> Vec<Zq<Q>> {
    // TODO: modulo the coefficients
    let gaussian = Normal::new(0.0, std_dev).unwrap();
    let val: Vec<Zq<Q>> = (0..dimension)
        .map(|_| Zq::<Q>::from(gaussian.sample(rng) as i64))
        .collect();

    val
}

pub fn get_gaussian<Rng: RngCore + CryptoRng, const Q: u64, const N: usize>(std_dev: f64, dimension: usize, rng: &mut Rng) -> Pow2CyclotomicPolyRing<Zq<Q>, N> {
    let rand_vec: Vec<Zq<Q>> = get_gaussian_vec(std_dev, dimension, rng)
        .try_into()
        .expect("Bad format");
    Pow2CyclotomicPolyRing::<Zq<Q>, N>::from(rand_vec)
}


// TODO: do I need both modules everywhere?
// move it to a dedicated library later on
pub struct PublicKey<const Q: u64, const P: u64, const N: usize> {
    // params: ParamsBFV,
    pub poly1: Pow2CyclotomicPolyRing<Zq<Q>, N>, 
    pub poly2: Pow2CyclotomicPolyRing<Zq<Q>, N>,
    pub modulo: u64,
}
pub struct SecretKey<const Q: u64, const P: u64, const N: usize> {
    // params: ParamsBFV,
    pub poly: Pow2CyclotomicPolyRing<Zq<P>, N>,  
    pub modulo: u64,
}

pub struct Plaintext<const P: u64, const N: usize> {
    // params: ParamsBFV,
    pub poly: Pow2CyclotomicPolyRing<Zq<P>, N>,
    pub modulo: u64,
}

pub struct Ciphertext<const Q: u64, const N: usize> {
    // params: ParamsBFV,
    pub c1: Pow2CyclotomicPolyRing<Zq<Q>, N>,
    pub c2: Pow2CyclotomicPolyRing<Zq<Q>, N>,
    pub modulo: u64,
}

pub fn q_to_p_ring<const Q: u64, const P: u64, const N: usize>(poly: Pow2CyclotomicPolyRing<Zq<Q>, N>) -> Pow2CyclotomicPolyRing<Zq<P>, N> {
    let coeffs: Vec<Zq<Q>> = poly.coeffs();
    let coeffs: Vec<Zq<P>> = coeffs.into_iter()
        .map(|x| <Zq<P>>::from(UnsignedRepresentative::from(x).0))
        .collect();
    let coeffs: [Zq<P>; N] = coeffs.try_into().expect("Bad format");
    Pow2CyclotomicPolyRing::<Zq<P>, N>::from(coeffs)
}
pub fn p_to_q_ring<const Q: u64, const P: u64, const N: usize>(poly: Pow2CyclotomicPolyRing<Zq<P>, N>) -> Pow2CyclotomicPolyRing<Zq<Q>, N> {
    let coeffs: Vec<Zq<P>> = poly.coeffs();
    let coeffs: Vec<Zq<Q>> = coeffs.into_iter()
        .map(|x| <Zq<Q>>::from(UnsignedRepresentative::from(x).0))
        .collect();
    let coeffs: [Zq<Q>; N] = coeffs.try_into().expect("Bad format");
    Pow2CyclotomicPolyRing::<Zq<Q>, N>::from(coeffs)
}

pub fn rand_ternary_poly<const P: u64, const N: usize>(size: usize) -> Pow2CyclotomicPolyRing<Zq<P>, N> {
    let bytes = Vector::<u8>::rand(size, &mut rand::thread_rng());
    
    WeightedTernaryChallengeSet::<Pow2CyclotomicPolyRing::<Zq<P>, N>>::try_from_random_bytes(bytes.as_slice()).unwrap()
}

impl<const Q: u64, const P: u64, const N: usize> SecretKey<Q, P, N> {
    pub fn new() -> Self {
        // generate random bytes to sample a ternary secret
        let size = WeightedTernaryChallengeSet::<Pow2CyclotomicPolyRing<Zq<P>, N>>::byte_size();

        Self {
            // params,
            poly: rand_ternary_poly(size),
            modulo: P,
        }
    }

    pub fn pk_gen(&self) -> PublicKey<Q, P, N> {
        let mut rng = rand::thread_rng();
        type Rq<const Q: u64, const N: usize> = Pow2CyclotomicPolyRing::<Zq<Q>, N>;
        let size = Pow2CyclotomicPolyRing::<Zq<Q>, N>::byte_size();
        
        // e should have small std_dev (how small?) for the correctness, TODO: check the parameters
        // TODO: use OpenFHE DGS
        let e = get_gaussian(1.0, size, &mut rng);

        // convert sk in Rp to Rq in order to perform the operation in Rq
        let sk_zq = p_to_q_ring(self.poly.clone());
        // compute the actual pk pair
        let pk2: Pow2CyclotomicPolyRing<Zq<Q>, N> = Rq::rand(&mut rng);
        let pk1: Pow2CyclotomicPolyRing<Zq<Q>, N> = -(pk2 * sk_zq + e);

        PublicKey {
            // params,
            poly1: pk1,
            poly2: pk2,
            modulo: Q,
        }
    }
    
    pub fn decrypt(&self, c: Ciphertext<Q, N>) -> Plaintext<P, N> {
        type Rp<const P: u64, const N: usize> = Pow2CyclotomicPolyRing::<Zq<P>, N>;
        let c1 = c.c1.clone();
        let c2 = c.c2.clone();
        let sk_zq = p_to_q_ring(self.poly.clone());
        let raw: Pow2CyclotomicPolyRing<Zq<Q>, N> = c1 + c2 * sk_zq;

        // convert the elements to the other field
        // let c1_zq = q_to_p_ring(c1);
        // let c2_zq = q_to_p_ring(c2);

        let p = self.modulo as f64;
        let q = c.modulo as f64;
        let delta = p/q; 

        let coeffs: Vec<Zq<P>> = raw
            .coeffs()
            .into_iter()
            .map(|x| <Zq<P>>::from((delta * (UnsignedRepresentative::from(x).0 as f64)) as u128))
            .collect();
        let coeffs: [Zq<P>; N] = coeffs.try_into().expect("Bad format");

        Plaintext {
            poly: Rp::from(coeffs),
            modulo: P,
        }

    }
}

impl<const Q: u64, const P: u64, const N: usize> PublicKey<Q, P, N> {
    pub fn encrypt(
        &self, 
        m: &Plaintext<P, N>, 
        r: (
            Pow2CyclotomicPolyRing<Zq<Q>, N>, 
            Pow2CyclotomicPolyRing<Zq<Q>, N>, 
            Pow2CyclotomicPolyRing<Zq<Q>, N>
            )
        ) -> Ciphertext<Q, N> {
        let (pk1, pk2) = (self.poly1.clone(), self.poly2.clone());
        let (u, e1, e2) = r;
        
        let p = m.modulo;
        let q = self.modulo;
        let delta = (q as f64 / p as f64).floor() as u128; // round off

        // mutliply each coeff of m with delta as bigint, and convert the polynomial into Zq, or convert m into Rq and multiply, but attention overflow
        // TODO: define it as scalar - poly multiplication 
        // let coeffs: Vec<Zq<P>> = m.poly.coeffs();

        let coeffs_zq: Vec<Zq<Q>> = m
            .poly
            .coeffs()
            .into_iter()
            .map(|x| <Zq<Q>>::from(delta * UnsignedRepresentative::from(x).0))
            .collect();
        let coeffs_zq: [Zq<Q>; N] = coeffs_zq.try_into().expect("Bad format");
        let m_delta= Pow2CyclotomicPolyRing::<Zq<Q>, N>::from(coeffs_zq);

        // compute a, b
        let c1 = pk1 * u.clone() + e1 + m_delta;
        let c2 = pk2 * u.clone() + e2;

        // return the ciphertext
        Ciphertext {
            c1,
            c2,
            modulo: Q,
        }
    }

    pub fn rand_tuple(factor: Option<Pow2CyclotomicPolyRing<Zq<Q>, N>>) 
    -> (Pow2CyclotomicPolyRing<Zq<Q>, N>, 
        Pow2CyclotomicPolyRing<Zq<Q>, N>, 
        Pow2CyclotomicPolyRing<Zq<Q>, N>) {
        type Rq<const Q: u64, const N: usize> = Pow2CyclotomicPolyRing::<Zq<Q>, N>;
        let mut rng = rand::thread_rng();
        let size = WeightedTernaryChallengeSet::<Pow2CyclotomicPolyRing<Zq<Q>, N>>::byte_size();

        match factor {
            Some(f) => (
                f * rand_ternary_poly(size), 
                f * get_gaussian(3.2, N, &mut rng), 
                f * get_gaussian(3.2, N, &mut rng)),
            None => (
                rand_ternary_poly(size), 
                get_gaussian(3.2, N, &mut rng),
                get_gaussian(3.2, N, &mut rng)),
        }
    }

}

impl<const P: u64, const N: usize> Plaintext<P, N> {
    pub fn rand_message() -> Self {
        Self {
            poly: Pow2CyclotomicPolyRing::<Zq<P>, N>::rand(&mut rand::thread_rng()),
            modulo: P,
        }
    }
}

#[test]
fn test() {
    const N: usize = 128;
    const Q: u64 = ntt_modulus::<N>(17);
    const P: u64 = ntt_modulus::<N>(15);
    // type Rq = Zq<Q>;
    // type Rp = Zq<P>;
    // type PolyRq = Pow2CyclotomicPolyRing<Rq, N>;
    // type PolyRp = Pow2CyclotomicPolyRing<Rp, N>;

    let sk: SecretKey<Q, P, N> = SecretKey::new();
    let pk = sk.pk_gen();
    let ptxt = Plaintext::rand_message();
    let ctxt = pk.encrypt(&ptxt, PublicKey::<Q, P, N>::rand_tuple(None));
    
    let act = sk.decrypt(ctxt).poly;
    // println!("act {act:?}");
    let exp = ptxt.poly;
    // println!("exp {exp:?}");
    assert_eq!(act, exp);
}