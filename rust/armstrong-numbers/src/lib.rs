pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();

    let num_str_len = num_str.len() as u32;

    let sum: u32 = num_str
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .map(|n| n.pow(num_str_len))
        .sum();

    sum == num
}
