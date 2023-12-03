use std::{collections::HashMap, io};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut grid = HashMap::from([((0, 0), 1u32)]);

    let mut x = 0;
    let mut y = 0;

    for curr in line.trim_end().chars() {
        match curr {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => {}
        }

        grid.entry((x, y)).and_modify(|val| *val += 1).or_insert(1);
    }

    let total = grid
        .values()
        .fold(0, |acc, curr| if *curr >= 1 { acc + 1 } else { acc });

    println!("{total}");
}
