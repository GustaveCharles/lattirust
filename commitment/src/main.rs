#[allow(dead_code)]

use lattirust_arithmetic::{ntt::ntt_modulus, ring::{Pow2CyclotomicPolyRing, Zq}};


const N: usize = 128;
const Q: u64 = ntt_modulus::<N>(16);
const P: u64 = ntt_modulus::<N>(15);
type Rq = Zq<Q>; 
type PolyRq = Pow2CyclotomicPolyRing<Rq, N>;
type Rp = Zq<P>; 
type PolyRp = Pow2CyclotomicPolyRing<Rp, N>;

fn main() {
    let mut rng = rand::thread_rng();
    // // let t = 12;         // Plaintext modulus
    // // let q = 65536;      // Ciphertext modulus
    // let std_dev = 1.0;  // Standard deviation for generating the error
    // // let degree = 4;     // Degree of polynomials used for encoding and encrypting messages

    // let v = get_gaussian_vec(std_dev, N, &mut rng);
    // println!("{:?}", v);

    // // construct the polynomial
    // let tmp: Vec<R> = v.into_iter().map(|x| R::from(x)).collect();
    // let poly = PolyR::from(tmp);
    // println!("{:?}", poly);
    // let mut rng = rand::thread_rng();
    // let coeff: Matrix<R> = Matrix::<R>::rand_ternary(1, N, &mut rng);
    // let coeff: Vector<R> = Vector::from_fn(N, |_, _| {
    //         [-R::one(), R::zero(), R::one()]
    //             .choose(&mut rng)
    //             .unwrap()
    //             .clone()
    //     });
    
    // let coeff = coeff.iter().map(|x| R::from(x)).collect();
    // let size = WeightedTernaryChallengeSet::<PolyRq>::byte_size();
    // let vec = Vector::<u8>::rand(size, &mut rng);
    // let vec = vec.as_slice();
    // let vec: PolyRq = WeightedTernaryChallengeSet::<PolyRq>::try_from_random_bytes(vec).unwrap();
    // let coeffs: Vec<Zq<Q>> = vec.coeffs();
    // let coeffs: Result<[Zq<Q>; N], _> = coeffs.try_into();
    // let coeffs = coeffs.unwrap();
    // let sk: PolyRq = PolyRq::from(coeffs);
    // println!("{:?}", sk);

    // let params = ParamsBFV::new(1, 2, 3);

    // let s = commitment::ppk::SecretKey<params.Q, params.P, params.N>::new(params);



}