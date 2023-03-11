#![allow(dead_code)]

// O(√n) returns true if n is a prime number.
fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}

// O(n log log n) returns a list of prime numbers less than or equal to n.
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }

        i += 1;
    }

    let mut primes = vec![];

    for (i, v) in is_prime.iter().enumerate() {
        if *v {
            primes.push(i);
        }
    }

    primes
}

// O(√n) returns a list of prime factors of n.
fn prime_factorize(mut n: usize) -> (Vec<usize>, std::collections::HashMap<usize, usize>) {
    let mut ans = vec![];
    let mut counts = std::collections::HashMap::new();

    for p in 2..=n {
        if p * p > n {
            break;
        }

        if n % p != 0 {
            continue;
        }

        let mut count = 0;

        while n % p == 0 {
            count += 1;
            n /= p;
        }

        ans.push(p);
        counts.insert(p, count);
    }

    if n != 1 {
        ans.push(n);
        counts.insert(n, 1);
    }

    (ans, counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        let n = 10;
        let mut primes = sieve_of_eratosthenes(n);
        primes.sort();
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_prime_factorize() {
        let n = 12;
        let (mut primes, counts) = prime_factorize(n);
        primes.sort();
        assert_eq!(primes, vec![2, 3]);
        assert_eq!(counts.get(&2), Some(&2));
        assert_eq!(counts.get(&3), Some(&1));
    }
}

fn main() {}
