use std::io::{self, Write};
use std::process::Command;

fn main() {
    if cfg!(target_os = "linux") {
        let output = Command::new("cat")
            .current_dir("/tmp")
            .arg("file.txt")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        // output the contents from the command
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        // assert
        //assert!(output.status.success());
    }
}
