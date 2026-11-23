
use rand::{Rng};

fn miller_rabin_primality_test(n:u128) -> bool{
    if n<= 1 {return false}
    else if n==2 {return true}
    else if n%2==0 {return false}
    else {  
        let k = 10000;
        let mut d = n;
        d -= 1;
        let mut s = 0u32;
        while d%2==0 {
            s += 1;
            d = d/2;
        }
        for _i in 0..k {
            let a = rand::thread_rng().gen_range(2..n-1);
            let mut x = mod_exp::mod_exp(a, d, n);
            
            for _j in 0..s {
                let y = x*x%n;
                if y==1 && x != 1 && x != n-1 {
                    return false;
                }
                x = y;
            }
            if x!=1{
                return false;
            }
        }
        return true;
    }
}

