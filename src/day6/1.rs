use regex::Regex;
use std::io;

fn main() {
    let mut grid = [[0u8; 1000]; 1000];

    let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let Some(caps) = re.captures(&line) else {
            panic!("invalid line")
        };

        let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let xx = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let yy = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();

        #[allow(clippy::needless_range_loop)]
        for i in x..=xx {
            for j in y..=yy {
                grid[i][j] = if line.starts_with("turn on") {
                    grid[i][j] + 1
                } else if line.starts_with("turn off") {
                    grid[i][j].saturating_sub(1)
                } else {
                    grid[i][j] + 2
                };
            }
        }
    }

    let total = grid
        .iter()
        .flat_map(|r| r.iter())
        .fold(0, |acc, curr| acc + *curr as u32);

    println!("{total}");
}
