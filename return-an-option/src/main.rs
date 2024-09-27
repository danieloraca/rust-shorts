fn the_function(return_ok: bool) -> Result<i32, &'static str> {
    if return_ok {
        return Ok(1);
    }

    Err("oh no, I'm returning an error")
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result_ok: Result<i32, &'static str> = the_function(true);
    let value: Option<i32> = result_ok.ok();

    match value {
        Some(val) => println!("Returned OK: {}", val),
        None => println!("Got nothing back"),
    }

    match divide(10, 0) {
        Ok(result) => println!("Got result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
