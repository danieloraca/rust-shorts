fn check(x: i32) -> bool {
    println!("{}", x);

    false
}

fn checker() {
    match (1, 2) {
        (x, _) | (_, x) if check(x) => println!("3"),
        _ => println!("4"),
    }
}

fn main() {
    checker();
}
