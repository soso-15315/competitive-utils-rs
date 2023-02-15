#![allow(dead_code)]

fn main() {}

// O(√n) returns a list of divisors of n.
fn divisors(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors() {
        let n = 12;
        let mut lst = divisors(n);
        lst.sort();
        assert_eq!(lst, vec![1, 2, 3, 4, 6, 12]);
    }
}
