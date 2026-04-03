pub fn is_prime(num: i64) -> bool {
    num > 1 && !(2..=num.isqrt()).any(|factor| num % factor == 0)
}

pub fn is_palindrome(input: &str) -> bool {
    input.chars().eq(input.chars().rev())
}

// return type: (factor, number of time)
pub fn prime_factorise(mut num: i64) -> Vec<(i64, u32)> {
    let mut vec = Vec::new();

    let ceil = num;

    for i in 2..=ceil {
        let mut count = 0;

        while num % i == 0 {
            count += 1;
            num /= i;
        }

        if count > 0 {
            vec.push((i, count));
        }
    }

    vec
}

#[cfg(test)]
mod test {
    use crate::{is_prime, prime_factorise};

    #[test]
    fn test_prime_factorise() {
        assert_eq!(prime_factorise(2), vec![(2, 1)]);
        assert_eq!(prime_factorise(8), vec![(2, 3)]);
        assert_eq!(prime_factorise(20), vec![(2, 2), (5, 1)]);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
    }
}
