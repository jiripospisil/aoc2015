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

            let l = parts[0];
            let w = parts[1];
            let h = parts[2];

            parts.sort_unstable();

            acc + 2 * l * w + 2 * w * h + 2 * h * l + parts[0] * parts[1]
        });

    println!("{total}");
}
