use std::io;
use std::fs;
use std::thread;
use std::str;
use std::io::Result;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

fn main() {
    let file_path = "VarOSLogger.json";
    if Path::new(file_path).exists() {
        userspace()
    }
    else {
        let contents = "first_boot = True";
        let init = fs::write(file_path, contents);
        println!("Initialized {}", file_path);
        match init {
            Ok(value) => {
                userspace();
            }
            Err(value) => {
                println!("Catastrophic Error!");
            }
        }
    }
}

fn userspace() {
    let mut cmd = Command::new("py");
    cmd.arg("Userspace\\OS.py");
    let passover = cmd.spawn().expect("Failure!");

}
