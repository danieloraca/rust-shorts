fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let f = add;

    let result: i32 = f(1, 2);
    println!("{}", result);
}
