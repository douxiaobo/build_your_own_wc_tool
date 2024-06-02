use std::fs;
use std::io::Read;

fn main() {
    let mut args = std::env::args();
    args.next(); // skip the first argument

    let mut command = String::from("a");
    let mut file_name = String::new();
    if let Some(arg) = args.next() {
        if arg.contains(".") {
            command = "a".to_string();
            file_name = arg.to_string();
        } else if arg == "-c" {
            command = "c".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else if arg == "-l" {
            command = "l".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else if arg == "-w" {
            command = "w".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else if arg == "-m" {
            command = "m".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else {
            println!("Invalid command");
            return;
        }
    }

    let mut contents = String::new();
    if file_name.is_empty() {
        let stdin = std::io::stdin();
        let result = stdin.lock().read_to_string(&mut contents);
        match result {
            Ok(_) => (), // Handle successful read from stdin (no explicit action needed)
            Err(err) => {
                println!("Error reading from stdin: {}", err);
                return;
            }
        }
    } else {
        contents = fs::read_to_string(file_name.clone()).unwrap_or_else(|err| {
            println!("Error reading file: {}", err);
            "".to_string()
        });
    }

    println!("{} {}", result(command, contents.clone()), file_name);
}

fn result(command: String, contents: String) -> String {
    match command.as_str() {
        "a" => {
            format!(
                "{}\t{}\t{}",
                contents.len(),
                contents.lines().count(),
                contents.split_whitespace().count()
            )
        }
        "c" => contents.len().to_string(),
        "l" => contents.lines().count().to_string(),
        "w" => contents.split_whitespace().count().to_string(),
        "m" => contents.chars().count().to_string(),
        _ => "Invalid command".to_string(),
    }
}