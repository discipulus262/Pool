use cursive::views::{Dialog, TextView};
use std::fs;
use std::process::Command;
pub fn push() {
    let _ga = Command::new("git").args(["add", "status.txt"]).output();
    let _result = Command::new("git")
        .args(["commit", "-m Status Changed"])
        .output()
        .expect("oops");
    let _gp = Command::new("git").args(["push"]).output().expect("oops");
}
pub fn pull() {
    let _result = Command::new("git").args(["pull"]).output().expect("oops");
}
pub fn write_open() {
    let _ = fs::write("status.txt", b"Open");
}
pub fn write_close() {
    let _ = fs::write("status.txt", b"Closed");
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::around(TextView::new("Open/Close the pool"))
            .title("Switch the Pool Status")
            .button("Open pool", |_s| write_open())
            .button("Close pool", |_s| write_close())
            .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();
    loop {
        pull();
        push();
    }
}
