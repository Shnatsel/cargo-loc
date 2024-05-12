use std::{ffi::OsString, process::Command};

fn main() {
    let cargo = std::env::var_os("CARGO").unwrap_or("cargo".into());
    let mut args: Vec<_> = std::env::args_os().skip(1).collect();
    if Some(&OsString::from("loc")) == args.first() {
        args.remove(0);
    }
    let status = Command::new(cargo)
        .arg("fetch")
        .args(args)
        .status()
        .expect("failed to invoke `cargo fetch`");
    if ! status.success() {
        // `cargo fetch` has already printed its error message, no need to add anything
        std::process::exit(status.code().unwrap_or(1));
    }
}
