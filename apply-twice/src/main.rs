fn apply_twice<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(value))
}

fn main() {
    let double = |x| x * 2;
    let value = apply_twice(double, 5);

    println!("The result is: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_twice() {
        let double = |x| x * 2;
        assert_eq!(apply_twice(double, 5), 20);
    }
}
