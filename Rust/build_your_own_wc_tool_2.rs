use std::fs;
use std::io::{ stdin,Read};

fn main() {
    let mut args = std::env::args();
    args.next(); // skip the first argument, which is the name of the program

    let mut command = "a".to_string(); // default command is 'a'
    let file_name_or_stdin = if let Some(arg) = args.next() {
        if arg.starts_with("-") {
            // It's a command flag
            match arg.as_str() {
                "-c" | "-l" | "-w" | "-m" => {
                    command = arg;
                    args.next().unwrap_or_else(|| "".to_string())
                },
                _ => {
                    println!("Invalid command");
                    return;
                }
            }
        } else {
            // It's a file name
            arg.to_string()
        }
    } else {
        "".to_string() // no file name or command provided
    };

    let contents = if file_name_or_stdin.is_empty() {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
        buffer
    } else {
        fs::read_to_string(&file_name_or_stdin).unwrap_or_else(|err| {
            println!("Error reading file: {}", err);
            "".to_string()
        })
    };

    println!("{} {}", result(command, contents), file_name_or_stdin);
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
        },
        "c" => contents.len().to_string(),
        "l" => contents.lines().count().to_string(),
        "w" => contents.split_whitespace().count().to_string(),
        "m" => contents.chars().count().to_string(),
        _ => "Invalid command".to_string(),
    }
}