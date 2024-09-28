use chrono::Local;

pub fn sleep_function() -> (String, String) {
    std::thread::sleep(std::time::Duration::from_secs(5));
    (
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_string(),
        "Woke up from sleep!\n".to_string(),
    )
}

pub fn time_function() -> (String, String) {
    let now = Local::now();
    let current_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    (
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_string(),
        format!("{}\n", current_time),
    )
}
