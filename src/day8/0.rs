use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r#"((?<string>\w+)|\\x\w{2}|\\\\|\\")"#).unwrap();

    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let escaped_len = curr.len();

            let unescaped_len = re.captures_iter(&curr).fold(0, |acc, curr| {
                if let Some(cap) = curr.name("string") {
                    return acc + cap.as_str().len();
                }

                acc + 1
            });

            acc + escaped_len - unescaped_len
        });

    println!("{total}");
}
