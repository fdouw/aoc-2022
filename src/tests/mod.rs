#[cfg(test)]
mod primes {
    use crate::lib_aoc::primes::PrimeSieve;

    #[test]
    fn test_primes() {
        let sieve = PrimeSieve::new();
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19],
            sieve.take_while(|p| p < &20).collect::<Vec<u64>>()
        );
    }
}
