use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r#"((?<string>\w+)|(?<hex>\\x\w{2})|\\\\|\\")"#).unwrap();

    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let unescaped_len = curr.len();

            let escaped_len = re.captures_iter(&curr).fold(6, |acc, curr| {
                if let Some(cap) = curr.name("string") {
                    return acc + cap.as_str().len();
                }

                if curr.name("hex").is_some() {
                    return acc + 5;
                }

                acc + 4
            });

            acc + escaped_len - unescaped_len
        });

    println!("{total}");
}
