pub fn raindrops(n: u32) -> String {
    let nums = get_factors_functional(n);
    if nums.contains(&3) {
        return "Pling".to_string();
    }

    if nums.contains(&5) {
        return "Plang".to_string();
    }

    if nums.contains(&7) {
        return "Plong".to_string();
    }

    n.to_string()
}

fn get_factors_functional(n: u32) -> Vec<u32> {
    (1..n + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<u32>>()
}
