use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|ws| format!("For want of a {} the {} was lost.\n", ws[0], ws[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}
