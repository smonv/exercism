pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;

    let mut prime_factors: Vec<u64> = Vec::new();

    while n > 1 {
        let mut f = 2;

        loop {
            if n % f == 0 {
                prime_factors.push(f);
                n = n / f;
                break;
            }

            f = f + 1;
        }
    }

    prime_factors
}
