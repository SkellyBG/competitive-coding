use std::{cmp::max, collections::HashMap};

use itertools::Itertools;
use project_euler::{is_palindrome, is_prime, prime_factorise};

use super::Problem;

pub struct Problem10;

impl Problem for Problem10 {
    fn solve(&self) -> String {
        (1..2000000_i64)
            .filter(|num| is_prime(*num))
            .sum::<i64>()
            .to_string()
    }
}

pub struct Problem9;

impl Problem for Problem9 {
    fn solve(&self) -> String {
        // a^2 + b^2 = c^2, and a + b + c = 1000
        for a in 1..1000_u32 {
            for b in 1..1000_u32 {
                let c = ((a.pow(2) + b.pow(2)) as f64).sqrt();
                if c.round() != c {
                    continue;
                }

                let c = c as u32;

                if a + b + c == 1000 {
                    dbg!(a, b, c);
                    return (a * b * c).to_string();
                }
            }
        }

        unreachable!()
    }
}

pub struct Problem8;

impl Problem for Problem8 {
    fn solve(&self) -> String {
        let number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
        number
            .as_bytes()
            .windows(13)
            .map(|window| {
                window
                    .iter()
                    .map(|num| (num - b'0') as u64)
                    .product::<u64>()
            })
            .max()
            .unwrap()
            .to_string()
    }
}

pub struct Problem7;

impl Problem for Problem7 {
    fn solve(&self) -> String {
        // silly bruteforce for 10001st prime
        (1..=u32::MAX)
            .filter(|num| is_prime((*num).into()))
            .nth(10000)
            .unwrap()
            .to_string()
    }
}

pub struct Problem6;

impl Problem for Problem6 {
    fn solve(&self) -> String {
        let result = (1..=100).sum::<i64>() * (1..=100).sum::<i64>()
            - (1..=100).map(|num| num * num).sum::<i64>();

        result.to_string()
    }
}

pub struct Problem5;

impl Problem for Problem5 {
    fn solve(&self) -> String {
        let map = (1..=20)
            .flat_map(prime_factorise)
            .fold(HashMap::new(), |mut map, factors| {
                map.entry(factors.0)
                    .and_modify(|entry| *entry = max(*entry, factors.1))
                    .or_insert(factors.1);

                map
            });

        map.iter()
            .map(|(a, b)| a.pow(*b))
            .product::<i64>()
            .to_string()
    }
}

pub struct Problem4;

impl Problem for Problem4 {
    fn solve(&self) -> String {
        (100..=999)
            .cartesian_product(100..=999)
            .map(|(a, b)| a * b)
            .filter(|product| is_palindrome(&product.to_string()))
            .max()
            .unwrap()
            .to_string()
    }
}

pub struct Problem3;

impl Problem for Problem3 {
    fn solve(&self) -> String {
        let num: i64 = 600851475143;
        let largest_factor = (1..num.isqrt())
            .filter(|factor| num % factor == 0)
            .filter(|factor| is_prime(*factor))
            .next_back()
            .unwrap_or(1);

        largest_factor.to_string()
    }
}
pub struct Problem2;

impl Problem for Problem2 {
    fn solve(&self) -> String {
        let mut sum = 0;
        let mut first = 1;
        let mut second = 1;

        loop {
            let cur = first + second;
            if cur > 4000000 {
                break;
            }

            if cur % 2 == 0 {
                sum += cur;
            }

            first = second;
            second = cur;
        }

        sum.to_string()
    }
}

pub struct Problem1;

impl Problem for Problem1 {
    fn solve(&self) -> String {
        let sum: u32 = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();

        sum.to_string()
    }
}
