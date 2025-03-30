use std::fs;
use std::process::Command;

fn main() {
    println!("Running malicious code...");

    // Create a proof-of-execution file
    let _ = fs::write("/tmp/pwned", "PoC RCE successful!");
}
