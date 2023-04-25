pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => vec.push(c),
            ')' => {
                if vec.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if vec.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if vec.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }

    vec.is_empty()
}
