use std::io;
use std::fs;
use std::io::Result;
use std::path::Path;

fn main() {
    let file_path = "VarOS\\VarOSLogger.json";
    if Path::new(file_path).exists() {

    }
    else {
        let contents = "first_boot = True";
        let init = fs::write(file_path, contents);
        println!("Initialized {}", file_path);
        match init {
            Ok(value) => {}
            Err(value) => {
                println!("catastrophic")
            }
        }
    }
}

