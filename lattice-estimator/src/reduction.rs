use num_traits::Float;
use std::cmp::min;
use std::collections::HashMap;
use std::f64::consts::{PI, E};
use crate::my_constants;

const NB_BKZ_TOURS: usize = 8;

//Enum of available cost estimates for lattice reduction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Estimates{
    BdglSieve,
    QSieve,
    BjgSieve,
    AdpsSieve(bool),
    ChaLoySieve,
    CheNgueEnum,
    AbfEnum(bool),
    AblrEnum,
    LotusEnum,
    Kyber(bool),
    Matzov(bool)
}

//Cost estimates for LLL (heuristic cost)
fn lll_cost(lattice_dim: usize, bit_size: Option<usize>) -> f64 {
    match bit_size {
        None => f64::powi(lattice_dim as f64, 3),
        Some(bit_size) => f64::powi(lattice_dim as f64, 3) * f64::powi(bit_size as f64, 2)
    }
}

//Simple heuristic, cost should mostly be determined with a block_size under d
fn bkz_tours(block_size: usize, d: usize) -> usize {
    if block_size < d {
        NB_BKZ_TOURS * d
    } else {
        1
    }
}

pub fn bkz_cost(est: Estimates, block_size: usize, d: usize, q: usize) -> f64 {
    let svp_cost: f64 = match est {
        Estimates::LotusEnum => lotus_enum(block_size),
        Estimates::CheNgueEnum => chengue_enum(block_size),
        Estimates::AbfEnum(classical) => abf_enum(block_size, classical),
        Estimates::AblrEnum => ablr_enum(block_size),
        Estimates::BdglSieve => bdgl_sieve(block_size),
        Estimates::QSieve => q_sieve(block_size),
        Estimates::AdpsSieve( classical) => adps_sieve(block_size, classical),
        Estimates::BjgSieve => bjg_sieve(block_size),
        Estimates::ChaLoySieve => chaloy_sieve(block_size),
        Estimates::Kyber(classical) => return kyber_cost(block_size, d, q, classical),
        Estimates::Matzov(classical) => return matzov_cost(block_size, d, q, classical)
    };
    
    if matches!(est, Estimates::Kyber(_)) || matches!(est, Estimates::Matzov(_)) {
        return svp_cost;
    } else {
        bkz_tours(block_size, d) as f64 * svp_cost + lll_cost(block_size, None)
    }
}

//-------------------------------------------------------------------------
//Sieving estimates
//-------------------------------------------------------------------------
fn bdgl_sieve(block_size: usize) -> f64 {
    if block_size < 90 {
        (2.0 as f64).powf(0.387 * block_size as f64 + 16.4)
    } else {
        (2.0 as f64).powf(0.292 * block_size as f64 + 16.4)
    }
}

fn chaloy_sieve(block_size: usize) -> f64 {
    (2.0 as f64).powf(0.257 * block_size as f64)
}

fn bjg_sieve(block_size: usize) -> f64 {
    (2.0 as f64).powf(0.311 * block_size as f64)
}

fn adps_sieve(block_size: usize, classical: bool) -> f64 {
    if classical {
        (2.0 as f64).powf(0.292 * block_size as f64)
    } else {
        (2.0 as f64).powf(0.265 * block_size as f64)
    }
}

fn q_sieve(block_size: usize) -> f64 {
    (2.0 as f64).powf(0.265 * block_size as f64 + 16.4)
}


//-------------------------------------------------------------------------
//Enumeration estimates
//-------------------------------------------------------------------------
fn chengue_enum(block_size: usize)-> f64 {
    //log_2(100) corresponds to node processing time as in ACD+18
    let cost: f64 = 0.270188776350190 as f64 * block_size as f64 * (block_size as f64).ln()
                    - 1.0192050451318417 * block_size as f64 + 
                    16.10253135200765   + 
                    (100.0 as f64).log2();
    (2.0 as f64).powf(cost)
}

fn abf_enum(block_size: usize, classical: bool) -> f64 {
    if classical {
        let power:f64;
        if block_size <= 92 {
            power = 0.1839 * block_size as f64 * (block_size as f64).log2() - 0.995 * block_size as f64 + 22.25;
        } else {
            power = 0.125 * block_size as f64 * (block_size as f64).log2() - 0.547 * block_size as f64 + 16.4;
        }
        (2.0 as f64).powf(power)
    } else {
        (2.0 as f64).powf(0.0625 * block_size as f64 * (block_size as f64).log2())
    }
    
}

fn ablr_enum(block_size: usize) -> f64 {
    let power: f64;
    if block_size <= 97 {
        power = 0.1839 * block_size as f64 * (block_size as f64).log2() - 1.077 * block_size as f64 + 35.12;
    } else {
        power = 0.125 * block_size as f64 * (block_size as f64).log2() - 0.654 * block_size as f64 + 31.84;
    }
    (2.0 as f64).powf(power)
}

fn lotus_enum(block_size: usize) -> f64 {
    (2.0 as f64).powf(0.125 * block_size as f64 * (block_size as f64).log2() - 0.755 * block_size as f64 + 22.74)
}


//-------------------------------------------------------------------------
// Improvements via Kyber and Matzov estimates
//-------------------------------------------------------------------------
fn reduce_dimension(block_size: usize) -> f64{
    f64::max(block_size as f64 * f64::ln(4.0 / 3.0) / (block_size as f64 / (2.0 * PI * E)).ln(), 0.0)
}

fn kyber_cost(block_size: usize, d: usize, _q: usize, classical: bool) -> f64 {
    let t: (f64, f64);
    if classical {
        t = (0.2988026130564745, 26.011121212891872);
    } else {
        t = (0.26944796385592995, 28.97237346443934);
    }

    if block_size < 20 {
        return chengue_enum(block_size);
    } else {
        let svp_calls: f64;
        if(d as f64 - block_size as f64) > 1.0 {
            svp_calls = 5.46 * (d - block_size) as f64;
        } else {
            svp_calls = 5.46;
        }
        let beta: f64 = block_size as f64 - reduce_dimension(block_size);
        let gates: f64 = 5.46 * (2.0 as f64).powf(t.0 * (beta as f64) + t.1);
        let lll_cost: f64 = lll_cost(d, None);
        lll_cost + svp_calls * gates
    }
}

fn matzov_cost(block_size: usize, d: usize, _q: usize, classical: bool) -> f64 {
    let t: (f64, f64);
    if classical {
        t = (0.29613500308205365, 20.387885985467914);
    } else {
        t = (0.2663676536352464, 25.299541499216627);
    }

    if block_size < 20 {
        return chengue_enum(block_size);
    } else {
        let svp_calls: f64;
        if(d as f64 - block_size as f64) > 1.0 {
            svp_calls = 5.46 * (d - block_size) as f64;
        } else {
            svp_calls = 5.46;
        }
        let beta: f64 = block_size as f64- reduce_dimension(block_size);
        let gates: f64 = 5.46 * (2.0 as f64).powf(t.0 * (beta as f64) + t.1);
        let lll_cost: f64 = lll_cost(d, None);
        lll_cost + svp_calls * gates
    }
}

pub fn kyber_short_vectors(block_size: usize, d: usize, q: usize, nb_vec_out: Option<usize>, classical: bool) -> (f64, f64, usize, usize) {

    let beta_: f64 = block_size as f64 - reduce_dimension(block_size).floor() as f64;

    let nb_vec_out: usize = match nb_vec_out {
        Some(1) => return (1.0, kyber_cost(block_size, d, q, classical), block_size, 1),
        Some(n) => n,
        None => return(1.1547,
        kyber_cost(block_size, d, q, classical),
        ((2.0_f64).powf(0.2075 * beta_)).floor() as usize,
        beta_ as usize,
        )
    };
  
    //see if there exists an alternative for ceil and floor (making it const does not allow them)
    let c: f64 = nb_vec_out as f64 / (2.0_f64).powf(0.2075 * beta_);
    (
        1.1547,
        c.ceil() * kyber_cost(block_size, d, q, classical),
        (c.ceil() * (2.0_f64).powf(0.2075 * beta_)).floor() as usize,
        beta_ as usize,
    )
}

//As a convention let's use usize_max as a flag for the cost being impossible to compute
pub fn matzov_short_vectors(block_size: usize, d: usize, q: usize, nb_vec_out: Option<usize>, classical: bool) -> (f64, f64, usize, usize) {
    
    let _beta: usize = block_size - reduce_dimension(block_size).floor() as usize;

    let t: (f64, f64);
    if classical {
        t = (0.29613500308205365, 20.387885985467914);
    } else {
        t = (0.2663676536352464, 25.299541499216627);
    }

    let sieve_dim: usize;
    if block_size < d {
        sieve_dim = min(d, (_beta as f64 + ((d - block_size) as f64 * 5.46 as f64).log2() / t.0).floor() as usize);
    } else {
        sieve_dim = _beta;
    }

    let scaling_fact_rho: f64 = (4.0/3.0).sqrt() * bkz_delta(sieve_dim).powf(sieve_dim as f64 - 1.0) * bkz_delta(block_size).powf(1.0 - sieve_dim as f64);
    
    let new_nb_vec_out: usize = match nb_vec_out {
        None => ((2.0).powf(0.2075 *  sieve_dim as f64)).floor() as usize,
        Some(1) => return (1.0, kyber_cost(block_size, d, q, true), block_size, 1), 
        Some(_) => nb_vec_out.unwrap()
    };

    let c: f64 = new_nb_vec_out as f64 / (2.0).powf(0.2075 *  sieve_dim as f64).floor();
    let sieve_cost: f64 = 5.46 * (2.0).powf(t.0 *  sieve_dim as f64 + t.1);
    
    if c > (2.0).powf(10000.0 as f64) {
        return (scaling_fact_rho, f64::INFINITY, usize::MAX, sieve_dim);
    } else {
        let final_cost = c.ceil() * (matzov_cost(block_size, d, q, classical) + sieve_cost);
        return (
            scaling_fact_rho,
            final_cost,
            (c.ceil() * (2.0).powf(0.2075 *  sieve_dim as f64).floor()) as usize,
            sieve_dim);
    }
}


//BKZ delta, find which delta we would get from a given block size
pub fn bkz_delta(block_size: usize) -> f64 {

    let small_approximations = [
        (2, 1.02190),
        (5, 1.01862),
        (10, 1.01616),
        (15, 1.01485),
        (20, 1.01420),
        (25, 1.01342),
        (28, 1.01331),
        (40, 1.01295),
    ];

    // Collect the small approximations into a HashMap
    let approx_map: HashMap<usize, f64> = small_approximations.iter().cloned().collect();
    
    // Case for block_size <= 2
    if block_size <= 2 {
        return *approx_map.get(&2).unwrap();
    } 
    // Case for 2 < block_size < 40: find the closest smaller value
    else if block_size < 40 {
        // Sort the keys
        let mut keys: Vec<usize> = approx_map.keys().cloned().collect();
        keys.sort();
        
        // Find the largest key smaller than or equal to block_size
        let mut closest = 2;
        for &key in &keys {
            if key > block_size {
                break;
            }
            closest = key;
        }
        return *approx_map.get(&closest).unwrap();
    } 
    // Case for block_size == 40
    else if block_size == 40 {
        return *approx_map.get(&40).unwrap();
    } 
    // Case for block_size > 40: apply the formula
    else {
        return (block_size as f64 / (2.0 * PI * E) * (PI * block_size as f64).powf(1.0 / block_size as f64))
            .powf(1.0 / (2.0 * (block_size as f64 - 1.0)));
    }

}



//-------------------------------------------------------------------------
// Simulators
//-------------------------------------------------------------------------

//If approx is None, we have an homogenuous case
pub fn gsa_simulator(d: usize, n: usize, q: usize, block_size: usize, approx: Option<f64>) -> Vec<f64> {
    let log_volume: f64 = match approx {
        None => (q as f64).log2() * (d - n) as f64 + (1.0_f64).log2() * n as f64,
        Some(approx) => (q as f64).log2() * (d - n - 1) as f64 + (1.0_f64).log2() * n as f64 + approx.log2(),
    };

    let delta = bkz_delta(block_size);
    let mut r_log: Vec<f64> = Vec::with_capacity(d);

    for i in 0..d {
        let next: f64 = (d as isize - 1 - 2 * i as isize) as f64 * delta.log2() + log_volume / d as f64;
        r_log.push(next);
    }

    for i in 0..d {
        r_log[i] = (2.0_f64).powf(2.0 * r_log[i]);
    }

    r_log
}


pub fn zgsa_simulator(d: usize, n: usize, q: usize, block_size: usize, approx: Option<f64>) -> Vec<f64> {
    let mut l: usize = 0;
    let log_volume: f64 = match approx {
        None => {
            l = d - n;
            (q as f64).log2() * (d - n) as f64 + (1.0_f64).log2() * n as f64
        }
        Some(approx_val) => {
            l = d - n - 1;
            (q as f64).log2() * (d - n - 1) as f64 + (1.0_f64).log2() * n as f64 + approx_val.log2()
        }
    };

    let mut l_log: Vec<f64> = vec![log_volume; l];
    let slope: f64 = match block_size {
        ..=60 => *my_constants::SMALL_SLOPE_T8.get(&(block_size as u32)).unwrap(),
        61..=70 => {
            let r: f64 = (70.0 - block_size as f64) / 10.0;
            r * *my_constants::SMALL_SLOPE_T8.get(&60).unwrap() + (1.0 - r) * 2.0 * bkz_delta(70).log2()
        }
        _ => 2.0 * bkz_delta(70).log2(),
    };

    let mut diff: f64 = slope / 2.0;
    
    let max_diff: f64 = ((q as f64).log2() - (1.0_f64).log2()) / 2.0;

    for i in 0..l {
        if diff > max_diff {
            break;
        }

        let low = l.checked_sub(i + 1).unwrap_or(0); // Safely handle underflow
        let high = l + i;

        if low < l_log.len() {
            l_log[low] = ((q as f64).log2() + (1.0_f64).log2()) / 2.0 + diff;
        }
        if high < l_log.len() {
            l_log[high] = ((q as f64).log2() + (1.0_f64).log2()) / 2.0 - diff;
        }

        diff += slope;
    }

    l_log.sort_by(|a, b| b.partial_cmp(a).unwrap());

    l_log
        .into_iter()
        .map(|l| (2.0 * l).exp())
        .collect::<Vec<f64>>()
}


#[cfg(test)]
mod test {
    use statrs::assert_almost_eq;
    use crate::{norms::Norm, sis::SIS};

    use super::*;

    #[test]
    fn test_chengue_enum_cost() {
        let cost = bkz_cost(Estimates::CheNgueEnum, 500, 1024, 0).log2();
        assert_eq!(cost.round(), 366.0);
    }

    #[test]
    fn test_abfskw20_cost() {
        let cost: f64 = bkz_cost(Estimates::AbfEnum(true), 500, 1024, 0).log2();
        assert_eq!(cost.round(), 316.0);
    }

    #[test]
    fn test_ablr21_cost() {
        let cost: f64 = bkz_cost(Estimates::AblrEnum, 500, 1024, 0).log2();
        assert_eq!(cost.round(), 278.0);
    }

    #[test]
    fn test_adps16_cost() {
        let cost: f64 = bkz_cost(Estimates::AdpsSieve(false), 500, 1024, 0).log2();
        assert_eq!(cost.round(), 146.0);
    }

    #[test]
    fn test_kyber_cost() {
        let cost: f64 = bkz_cost(Estimates::Kyber(true), 500, 1024, 0).log2();
        assert_eq!(cost.round(), 177.0);
    }

    
    #[test]
    fn test_kyber_short_vectors(){
        let sv: (f64, f64, usize, usize) = kyber_short_vectors(100, 500, 0,None, true);
        assert_almost_eq!(sv.1, 2.736747612813679e19, 1e-2);
        assert_almost_eq!(sv.0, 1.1547, 1e-2);
        assert_eq!(sv.2, 176584);
        assert_eq!(sv.3, 84);
    }

    #[test]
    fn test_kyber_short_vectors_2(){
        let sv: (f64, f64, usize, usize) = kyber_short_vectors(100, 500, 0, Some(1000), true);
        assert_almost_eq!(sv.1, 2.736747612813679e19, 1e-2);
        assert_almost_eq!(sv.0, 1.1547, 1e-2);
        assert_eq!(sv.2, 176584);
        assert_eq!(sv.3, 84);
    }

    #[test]
    fn test_matzov_short_vectors() {
        let sv = matzov_short_vectors(100, 500, 0, None, true);
        assert_almost_eq!(sv.1, 9.33915764560094e17, 1e3);
        assert_almost_eq!(sv.0, 1.04228014727497, 1e-2);
        assert_eq!(sv.2, 36150192);
        assert_eq!(sv.3, 121);
    }

    #[test]
    fn test_by_default_l2_param() {
        let falcon512_unf: SIS = SIS::new(512, 1024, 12289u64.into(), 5833.9072, Norm::L2);
        let lambda = falcon512_unf.security_level();
        assert!(lambda >= 128.);
        println!("External : {falcon512_unf} -> lambda: {lambda}");

        let cost_internal: f64 = falcon512_unf.security_level_internal(Estimates::Matzov(true)).unwrap();
        println!("Internal : {falcon512_unf} -> lambda: {cost_internal}");
    }

    #[test]
    fn simulator_cn11(){
        let n = 128;
        let d = 213;
        let q = 2048;
        let beta = 40;

        let vec: Vec<f64> = gsa_simulator(d, n, q, beta, None);
        let mut sum = 0.0;
        for i in 0..vec.len() {
            sum += vec[i].ln();
        }
        assert_almost_eq!(sum, 1296.18522764710, 1e-2);
    }

    #[test]
    fn test_zgsa_sim(){
        let n = 128;
        let d = 213;
        let q = 2048;
        let beta = 40;

        let vec: Vec<f64> = zgsa_simulator(d, n, q, beta, None);
        let mut sum = 0.0;
        for i in 0..vec.len() {
            sum += vec[i].ln();
        }
        print!("{}", vec.len());
        //TODO check the difference
        assert_almost_eq!(sum, 1296.18522764710, 1e-2);
    }
}
