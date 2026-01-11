use std::io;
use std::fs;
use std::process;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Kernel control start
    let file_path = "VarOSLogger.json";
    if Path::new(file_path).exists() {
        userspace()
    }
    else {
        let contents = "first_boot = True";
        let init = fs::write(file_path, contents);
        println!("Initialized {}", file_path);
        match init {
            Ok(_value) => {
                userspace();
            }
            Err(_value) => {
                println!("Catastrophic Error!");
            }
        }
    }
    // Kernel control stop (until a log off or termination)
}

// Launch userspace and hand control over
fn userspace() {
    // The actual userspace
    let mut binding = Command::new("py");
    let cmd = binding.arg("Userspace\\OS.py");
    let mut passover = cmd.spawn().expect("Failure!");
    passover.wait().expect("damn u right");

    // Logoff and error handler
    let statuscode = fs::read_to_string("Userspace\\Temp\\statuscode.stat").expect("lol").parse::<i32>();
    if statuscode.unwrap() == 0 {
        println!("Are you sure you want to log off? (y/n)");
        let mut tempstring1 = String::new();
        io::stdin().read_line(&mut tempstring1).expect("Failed to read!");
        if tempstring1.trim() == "y" {
            println!("OS Exited.");
            thread::sleep(Duration::from_secs(1));
            process::exit(0);
        }
        else if tempstring1.trim() == "n" {
            userspace();
        }
        thread::sleep(Duration::from_secs(1));
    }
    else {
        println!("A catastrophic and unrecoverable error has occured.");
        thread::sleep(Duration::from_secs(1));
    }
}