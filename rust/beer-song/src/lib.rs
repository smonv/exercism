pub fn verse(n: u32) -> String {
    if n < 1 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else {
        let w1 = match n {
            2..=u32::MAX => format!("{} bottles", n),
            1 => "1 bottle".to_string(),
            0 => unreachable!(),
        };

        let l = n - 1;

        let w = match l {
            2..=u32::MAX => format!("{} bottles", l),
            1 => "1 bottle".to_string(),
            0 => "no more bottles".to_string(),
        };

        format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", w1 ,w1,if n > 1 {"one"} else {"it"}, w)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let range = if start < end {
        (start..end).collect::<Vec<_>>()
    } else {
        println!("{:?}", (end..=start).collect::<Vec<_>>());
        (end..=start).rev().collect::<Vec<_>>()
    };

    println!("{:?}", range);

    range
        .iter()
        .map(|&n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
