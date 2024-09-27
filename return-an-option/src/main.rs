fn the_function(return_ok: bool) -> Result<i32, &'static str> {
    if return_ok {
        return Ok(1);
    }

    Err("oh no, I'm returning an error")
}

fn main() {
    let result_ok: Result<i32, &'static str> = the_function(true);
    let value: Option<i32> = result_ok.ok();

    match value {
        Some(val) => println!("Returned OK: {}", val),
        None => println!("Got nothing back"),
    }
}
