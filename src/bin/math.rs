#![allow(dead_code)]

// O(log(max(a, b))) returns the greatest common divisor of a and b.
fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}

// O(log(max(a, b))) returns the least common multiple of a and b.
fn icm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

// O(k) binomial coefficient
fn binomial(n: isize, k: isize) -> isize {
    let mut ret: isize = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    return ret;
}

// O(1) rotation matrix
fn rotate(x: f64, y: f64, d: f64) -> (f64, f64) {
    let q = d.to_radians();

    let x2 = q.cos() * x + -q.sin() * y;
    let y2 = q.sin() * x + q.cos() * y;

    (x2, y2)
}

// to_base_string(10, 2) -> "1010"
fn to_base_string(mut n: usize, base: usize) -> String {
    let mut ans = vec![];
    loop {
        let a = n % base;
        let b = n / base;

        ans.push(format!("{}", a));
        if b == 0 {
            break;
        }
        n = b;
    }

    ans.reverse();
    ans.join("")
}

// O(log(b)) returns a^b mod m.
fn mod_power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    // change
    for i in 0..30 {
        let w = 1 << i;
        if (b & w) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }

    ans
}

fn division(a: usize, b: usize, m: usize) -> usize {
    (a * mod_power(b, m - 2, m)) % m
}

fn ncr_m(n: usize, r: usize, m: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    for i in 1..=n {
        a = a * i % m;
    }

    for i in 1..=r {
        b = b * i % m;
    }

    for i in 1..=(n - r) {
        b = b * i % m;
    }

    division(a, b, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
    }

    #[test]
    fn test_icm() {
        assert_eq!(icm(12, 18), 36);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2), 10);
    }

    #[test]
    fn test_to_base_string() {
        assert_eq!(to_base_string(10, 2), "1010");
    }

    #[test]
    fn test_mod_power() {
        assert_eq!(mod_power(2, 10, 1_000_000_007), 1024);
    }

    #[test]
    fn test_division() {
        assert_eq!(division(2, 3, 1_000_000_007), 666666672);
    }

    #[test]
    fn test_ncr_m() {
        assert_eq!(ncr_m(10, 2, 1_000_000_007), 45);
    }
}

fn main() {}
