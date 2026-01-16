use std::io;
use std::fs;
use std::process;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Kernel control start
    let file_path = "Userspace\\Transfer Pipelines\\Credentials\\username.json";
    let file_path2 = "Userspace\\Transfer Pipelines\\Credentials\\password.json";
    let file_path3 = "Userspace\\Transfer Pipelines\\unsafe_mode.json";
    if Path::new(file_path).exists() && Path::new(file_path2).exists() && Path::new(file_path3).exists() {
        userspace();
    }
    else {
        let mut username = String::new();
        let mut password = String::new();
        let mut answer = String::new();
        println!("It seems to be your first time here!");
        println!("Let's get you acquainted!");
        println!("First things first, username and password!");
        println!("What would you like your username to be?");
        io::stdin().read_line(&mut username).expect("Failed to read username value!");
        let init = fs::write(file_path, username.trim());
        println!("Next, what would you like your password to be?");
        io::stdin().read_line(&mut password).expect("Failed to read username value!");
        let init2 = fs::write(file_path2, password.trim());
        println!("Now, would you like unsafe mode on your OS?");
        println!("This will give you more control, but you'll likely get more unrecoverable errors.");
        println!("Would you? (1 = yes/0 = no)");
        io::stdin().read_line(&mut answer).expect("Failed to read answer!");
        let init3 = fs::write(file_path3, answer.trim());

        match init {
            Ok(_value) => {
                match init2 {
                    Ok(_value) => {
                        match init3 {
                            Ok(_value) => {
                                println!("Initialized {}", file_path);
                                println!("Initialized {}", file_path2);
                                println!("Initialized {}", file_path3);
                                userspace();
                            }

                            Err(_value) => {
                                println!("Catastrophic Failure, Unsafe Mode Read")
                            }
                        }
                    }

                    Err(_value) => {
                        println!("Catastrophic Error, Password Read")
                    }
                }
            }

            Err(_value) => {
                println!("Catastrophic Error, Username Read");
            }
        }
    }
}
// Kernel Control Stop

// Launch userspace and hand control over
fn userspace() {
    loop {
        // The actual userspace
        let mut binding = Command::new("py");
        let cmd = binding.arg("Userspace\\OS.py");
        let mut passover = cmd.spawn().expect("Failure!");
        passover.wait().expect("Userspace transfer failed.");

        // Logoff and error handler
        let statuscode = fs::read_to_string("Userspace\\Transfer Pipelines\\statuscode.stat").expect("lol").parse::<i32>().unwrap(); // Reads the status code from the statuscode channel
        match statuscode { // This match is essentially a granular error handler with 4000000000 error codes to work with.
            0 => { // Successful error code, meaning a log off
                println!("Are you sure you want to log off? (y/n)");
                let mut tempstring1 = String::new();
                io::stdin().read_line(&mut tempstring1).expect("Failed to read!");
                if tempstring1.trim() == "y" {
                    println!("OS Exited.");
                    kernelexit();
                }
                else if tempstring1.trim() == "n" {
                    continue;
                }
                else {
                    println!("Line read failed!");
                    continue;
                }
            }

            1 => {
                println!("Too many wrong attempts!");
                kernelexit();
            }

            _ => { // VarOS's BSOD, unexpected errors
                println!("Catastrophic Error!");
                println!(":c");
                println!(" ");
                println!("A catastrophic and unrecoverable error has occured.");
                println!("If you have encountered this error, contact Varanslash on GitHub or one of the other devs and explain the process to get to this error.");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn kernelexit() {
    thread::sleep(Duration::from_secs(1));
    process::exit(0);
}