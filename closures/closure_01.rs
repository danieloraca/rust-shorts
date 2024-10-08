fn main() {
    let x: i32 = 20;
    let get_answer: &dyn Fn(i32) -> i32 = &|y: i32| x + y;
    println!("{}", get_answer(10));
}
