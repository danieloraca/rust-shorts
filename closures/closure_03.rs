fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calc_and_print(x: i32, y: i32, calculator: fn(i32, i32) -> i32) {
        let result: i32 = calculator(x, y);
        println!("{}", result);
    }

    calc_and_print(1, 2, add);
    calc_and_print(1, 2, |x: i32, y: i32| x + y);
}
