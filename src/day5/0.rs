use itertools::Itertools;
use std::io;

fn is_nice(line: &str) -> bool {
    // 1
    if line.chars().fold(0, |acc, curr| match curr {
        'a' | 'e' | 'i' | 'o' | 'u' => acc + 1,
        _ => acc,
    }) < 3
    {
        return false;
    }

    // 2
    if !line.chars().tuple_windows().any(|(a, b)| a == b) {
        return false;
    }

    // 3
    if line
        .chars()
        .tuple_windows()
        .any(|(a, b)| matches!((a, b), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
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
