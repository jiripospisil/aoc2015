use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut level = 0;
    for (idx, curr) in line.trim_end().chars().enumerate() {
        level += if curr == '(' { 1 } else { -1 };

        if level == -1 {
            println!("{}", idx + 1);
            return;
        }
    }
}
