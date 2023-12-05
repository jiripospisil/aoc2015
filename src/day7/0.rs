use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug)]
enum WireOrLiteral {
    Literal(u16),
    Connection(String),
}

#[derive(Debug)]
enum Connection {
    Plain(String, String),
    And(WireOrLiteral, String, String),
    Or(String, String, String),
    Shift(bool, String, u16, String),
    Neg(String, String),
}

macro_rules! n {
    ($caps:ident, $idx:literal) => {
        $caps.get($idx).unwrap().as_str().parse::<u16>().unwrap()
    };
}

macro_rules! s {
    ($caps:ident, $idx:literal) => {
        $caps.get($idx).unwrap().as_str()
    };
}

macro_rules! so {
    ($caps:ident, $idx:literal) => {
        $caps.get($idx).unwrap().as_str().to_owned()
    };
}

fn main() {
    let assign = Regex::new(r"^(\d+) -> (\w+)$").unwrap();
    let connection = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
    let and_or = Regex::new(r"^(\w+) (AND|OR) (\w+) -> (\w+)$").unwrap();
    let shifts = Regex::new(r"^(\w+) (LSHIFT|RSHIFT) (\d+) -> (\w+)$").unwrap();
    let neg = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();

    let mut wires = HashMap::new();
    let mut connections = Vec::new();

    // ugh
    for line in io::stdin().lines().map(|line| line.unwrap()) {
        if let Some(caps) = assign.captures(&line) {
            wires.insert(so!(caps, 2), n!(caps, 1));
        } else if let Some(caps) = connection.captures(&line) {
            connections.push(Connection::Plain(so!(caps, 1), so!(caps, 2)));
        } else if let Some(caps) = and_or.captures(&line) {
            let name_or_literal = s!(caps, 1);
            let wrapped = if let Ok(num) = name_or_literal.parse::<u16>() {
                WireOrLiteral::Literal(num)
            } else {
                WireOrLiteral::Connection(name_or_literal.to_owned())
            };
            connections.push(if s!(caps, 2).starts_with("A") {
                Connection::And(wrapped, so!(caps, 3), so!(caps, 4))
            } else {
                Connection::Or(so!(caps, 1), so!(caps, 3), so!(caps, 4))
            });
        } else if let Some(caps) = shifts.captures(&line) {
            connections.push(Connection::Shift(
                s!(caps, 2).starts_with("L"),
                so!(caps, 1),
                n!(caps, 3),
                so!(caps, 4),
            ));
        } else if let Some(caps) = neg.captures(&line) {
            connections.push(Connection::Neg(so!(caps, 1), so!(caps, 2)));
        } else {
            panic!("{}", line);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for connection in &connections {
            match connection {
                Connection::Plain(a, b) => {
                    let Some(val) = wires.get(a) else {
                        continue;
                    };

                    if wires.contains_key(b) {
                        continue;
                    }

                    wires.insert(b.to_owned(), *val);
                    changed = true;
                }
                Connection::And(a, b, c) => {
                    let Some(val_a) = (match a {
                        WireOrLiteral::Literal(l) => Some(l),
                        WireOrLiteral::Connection(c) => wires.get(c),
                    }) else {
                        continue;
                    };

                    let Some(val_b) = wires.get(b) else {
                        continue;
                    };

                    if wires.contains_key(c) {
                        continue;
                    }

                    wires.insert(c.to_owned(), *val_a & *val_b);
                    changed = true;
                }
                Connection::Or(a, b, c) => {
                    let Some(val_a) = wires.get(a) else {
                        continue;
                    };

                    let Some(val_b) = wires.get(b) else {
                        continue;
                    };

                    if wires.contains_key(c) {
                        continue;
                    }

                    wires.insert(c.to_owned(), *val_a | *val_b);
                    changed = true;
                }
                Connection::Shift(left, a, s, b) => {
                    let Some(val_a) = wires.get(a) else {
                        continue;
                    };

                    if wires.contains_key(b) {
                        continue;
                    }

                    wires.insert(b.to_owned(), if *left { *val_a << s } else { *val_a >> s });
                    changed = true;
                }
                Connection::Neg(a, b) => {
                    let Some(val_a) = wires.get(a) else {
                        continue;
                    };

                    if wires.contains_key(b) {
                        continue;
                    }

                    wires.insert(b.to_owned(), !*val_a);
                    changed = true;
                }
            }
        }
    }

    dbg!(&wires.get("a"));
}
