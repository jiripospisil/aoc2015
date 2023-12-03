use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let level = line
        .trim_end()
        .chars()
        .fold(0, |acc, curr| if curr == '(' { acc + 1 } else { acc - 1 });

    println!("{level}");
}
