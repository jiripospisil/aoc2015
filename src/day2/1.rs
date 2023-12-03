use std::io;

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let mut parts = curr
                .split('x')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            parts.sort_unstable();

            acc + parts[0] * 2 + parts[1] * 2 + parts.iter().product::<u32>()
        });

    println!("{total}");
}
