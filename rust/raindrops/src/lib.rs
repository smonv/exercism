pub fn raindrops(n: u32) -> String {
    let factors: Vec<u32> = vec![3, 5, 7];

    let str = factors
        .iter()
        .filter(|&x| n % x == 0)
        .map(|x| match x {
            3 => "Pling".to_string(),
            5 => "Plang".to_string(),
            7 => "Plong".to_string(),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");

    if str.is_empty() {
        return n.to_string();
    }

    str
}
