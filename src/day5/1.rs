use itertools::Itertools;
use std::io;

fn is_nice(line: &str) -> bool {
    if line.chars().tuple_windows::<(char, char)>().all_unique() {
        return false;
    }

    if line
        .chars()
        .tuple_windows::<(char, char, char)>()
        .any(|(a, b, c)| a == b && a == c)
    {
        return false;
    }

    if !line
        .chars()
        .tuple_windows::<(char, char, char)>()
        .any(|(a, _, c)| a == c)
    {
        return false;
    }

    true
}

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| if is_nice(&curr) { acc + 1 } else { acc });

    println!("{total}");
}
