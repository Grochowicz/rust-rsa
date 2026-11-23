use std::io;
use rand::Rng;
use mod_exp::mod_exp;
use std::collections::BTreeMap;

// 1
fn to_fahrenheit(c:f64) -> f64 {
    return c*1.8 + 32.0;
}

// 2
fn fib(n:i32) {
    let mut a : i32 = 1;
    let mut b : i32 = 1;
    if n <= 0 {
        return;
    }
    println!("{}",a);
    if n <= 1 {
        return;
    }
    println!("{}",b);
    if n <= 2 {
        return;
    }
    for _i in 2..n {
        a += b;
        std::mem::swap(&mut a,&mut b);
        println!("{}",b);
    }
    return;
}

// 3
fn getmax(arr: &[i32] ) -> Option::<i32> {
    if arr.len() == 0 {
        return None;
    }
    let mut mx = arr[0];
    for x in arr {
        mx = std::cmp::max(mx, *x);
    }
    return Some(mx);
}

// 4
fn is_prime_brute(n:i128) {
    let mut i : i128 = 2;
    while i*i <= n {
        if n % i == 0 {
            println!("{} não é primo.",n);
            return;
        }
        i += 1;
    }
    println!("{} é primo!",n);
}
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

// 5
fn is_palindrome(s:&str) {
    let n : usize = s.len();
    let mut ok : bool = true;
    for i in 0..n {
        ok &= s.chars().nth(i).unwrap() == s.chars().nth(n-i-1).unwrap();
    }
    if ok {
        println!("{} é palíndromo!",s);
    }
    else {
        println!("{} não é palíndromo.",s);
    }
}

// 6
fn is_prime_brute6(n:u128) -> bool {
    let mut i : u128 = 2;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
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
fn is_prime(x : u128) -> bool {
    if x <= 1000000000000000 {
        return is_prime_brute6(x);
    }
    else {
        return is_prime_prob6(x);
    }
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

// 7
fn mais_frequente(s:&str) -> char {
    let mut f : BTreeMap<char,u32> = BTreeMap::new();
    for c in s.chars() {
        f.entry(c).or_insert(0);
        *f.get_mut(&c).unwrap() += 1;
    }
    /*
    for c in s.chars() {
        if let Some(x) = f.get_mut(&c) {
            *x += 1;
        }
        else {
            f.insert(c, 1);
        }
    }
    */
    let mut mx : u32 = 0;
    let mut ret : char = 'a';
    for (k,v) in f.into_iter() {
        if v > mx {
            mx = v;
            ret = k;
        }
    }
    return ret;
}

fn test_1() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = input.trim().parse::<f64>();
    if result.is_ok() {
        let val = result.unwrap();
        println!("{}",to_fahrenheit(val));
    }
    else {
        println!("Insira um número válido!");
    }
    Ok(())
}

fn test_2() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = input.trim().parse::<i32>();
    if result.is_ok() {
        let val = result.unwrap();
        fib(val);
    }
    else {
        println!("Insira um número válido!");
    }
    Ok(())
}

fn test_3() {
    let arr : &[i32] = &[14,13,17,73,21];
//    let arr : &[i32] = &[];
    let res : Option::<i32> = getmax(arr);
    match res {
        None => println!("Array vazio!"),
        Some(x) => println!("Máximo: {}",x)
    }
}

fn test_4() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = input.trim().parse::<i128>();
    if result.is_ok() {
        let val = result.unwrap();
        if val <= 1000000000000000 {
            is_prime_brute(val);
            is_prime_prob(val);
        }
        else {
            is_prime_prob(val);
       }
    }
    else {
        println!("Insira um número válido!");
    }
    Ok(())
}

fn test_5() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = input.trim();
    is_palindrome(result);
    Ok(())
}

fn test_6() {
    let v : Vec::<u128> = vec![14,13,17,73,21,998244353,998244357];
    let res : Vec::<u128> = filter_primes(v);
    for u in res {
        println!("{}",u);
    }
}

fn test_7() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result = input.trim();
    let res : char = mais_frequente(result);
    println!("Letra mais frequente: {}",res);
    Ok(())
}

fn main() {
    test_1();
//    test_2();
//    test_3();
//    test_4();
//    test_5();
//    test_6();
//    test_7();
}


