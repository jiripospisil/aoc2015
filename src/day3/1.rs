use std::{collections::HashMap, io};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut grid = HashMap::from([((0, 0), 2u32)]);

    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    for (idx, curr) in line.trim_end().chars().enumerate() {
        let x = if idx % 2 == 0 {
            &mut santa_x
        } else {
            &mut robo_x
        };

        let y = if idx % 2 == 0 {
            &mut santa_y
        } else {
            &mut robo_y
        };

        match curr {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => {}
        }

        grid.entry((*x, *y))
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    let total = grid
        .values()
        .fold(0, |acc, curr| if *curr >= 1 { acc + 1 } else { acc });

    println!("{total}");
}
