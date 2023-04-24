use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&x| x != 0)
        .flat_map(|&factor| (1..=(limit / factor)).map(move |x| factor * x))
        .filter(|&x| x < limit)
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
