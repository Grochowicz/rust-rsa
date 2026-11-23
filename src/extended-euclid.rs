fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (g,x,y) = ext_gcd(b, a%b);
    return (g, y, x - y * (a/b));
}
