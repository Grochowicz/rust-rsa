use std::io;
use rand::Rng;
use mod_exp::mod_exp;
use std::collections::BTreeMap;

fn is_prime_prob(n:i128) {
    // teste de primalidade de fermat
    let k : i32 = 10000;
    for _i in 0..k {
        let a : i128 = rand::thread_rng().gen_range(1..n);
        let p = mod_exp(a, n-1, n);
        if p != 1 {
            println!("{} não é primo.",n);
            return;
        }
    }
    println!("{} provavelmente é primo!",n);
}

fn is_prime_prob6(n:u128) -> bool {
    // teste de primalidade de fermat
    let k : u128 = 10000;
    for _i in 0..k {
        let a : u128 = rand::thread_rng().gen_range(1..n);
        let p = mod_exp(a, n-1, n);
        if p != 1 {
            return false;
        }
    }
    return true;
}

fn filter_primes(v : Vec::<u128>) -> Vec::<u128> {
    let mut ret : Vec::<u128> = Vec::<u128>::new();
    for u in &v {
        if is_prime(*u) {
            ret.push(*u);
        }
    }
    return ret;
}
