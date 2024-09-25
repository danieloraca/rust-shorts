pub fn crt_image() -> std::io::Result<()> {
    println!("Server listening on port 8080");
    let banner = format!(r#"--------"#);

    println!("{}", banner);

    Ok(())
}
