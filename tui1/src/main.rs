use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};
use std::process::Command;

fn main() {
    let mut siv = Cursive::default();

    let output = Command::new("ls")
        .args(&["-l", "-a"])
        .output()
        .expect("Failed to execute.");

    let output_str = String::from_utf8_lossy(&output.stdout);

    siv.add_layer(
        Dialog::around(TextView::new(output_str))
            .title("Command output")
            .button("Close", |s| s.quit()),
    );

    siv.run();
}
