use std::fs;
use std::io::Read;

/**
 * The main function, the entry point of the program.
 * It processes command line arguments to determine the command and the target file name, then reads the content accordingly,
 * and finally calls the result function to calculate and output the result.
 */
fn main() {
    // Get command line arguments
    let mut args = std::env::args();
    // Skip the first argument, which is the program name itself
    args.next(); // skip the first argument

    // Initialize command and file name variables
    let mut command = String::from("a");
    let mut file_name = String::new();

    // Process command line arguments to determine the command and file name
    if let Some(arg) = args.next() {
        if arg.contains(".") {
            // If the argument contains a dot, treat it as a file name directly
            command = "a".to_string();
            file_name = arg.to_string();
        } else if arg == "-c" {
            // If the argument is -c, set the command to "c" and get the next argument as the file name
            command = "c".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else if arg == "-l" {
            // If the argument is -l, set the command to "l" and get the next argument as the file name
            command = "l".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else if arg == "-w" {
            // If the argument is -w, set the command to "w" and get the next argument as the file name
            command = "w".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else if arg == "-m" {
            // If the argument is -m, set the command to "m" and get the next argument as the file name
            command = "m".to_string();
            file_name = args.next().unwrap_or_else(|| "".to_string());
        } else {
            // If the argument does not match any condition, output error message and exit
            println!("Invalid command");
            return;
        }
    }

    // Initialize the content variable to store the read content
    let mut contents = String::new();
    // Read content based on whether the file name is empty
    if file_name.is_empty() {
        // If the file name is empty, read from standard input
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
        // If the file name is not empty, read from the specified file
        contents = fs::read_to_string(file_name.clone()).unwrap_or_else(|err| {
            println!("Error reading file: {}", err);
            "".to_string()
        });
    }

    // Call the result function to calculate the result based on the command, then output
    println!("{} {}", result(command, contents.clone()), file_name);
}

/**
 * Calculates and returns the result based on the command and content.
 * 
 * @param command The command string, indicating the type of calculation to perform.
 * @param contents The content string, the data used for calculation.
 * @return A string representing the calculation result.
 */
fn result(command: String, contents: String) -> String {
    match command.as_str() {
        "a" => {
            // For command "a", calculate and return the length of the content, the number of lines, and the number of whitespace-separated words
            format!(
                "{}\t{}\t{}",
                contents.len(),
                contents.lines().count(),
                contents.split_whitespace().count()
            )
        }
        "c" => contents.len().to_string(), // For command "c", return the length of the content
        "l" => contents.lines().count().to_string(), // For command "l", return the number of lines in the content
        "w" => contents.split_whitespace().count().to_string(), // For command "w", return the number of whitespace-separated words in the content
        "m" => contents.chars().count().to_string(), // For command "m", return the number of characters in the content
        _ => "Invalid command".to_string(), // If the command is invalid, return an error message
    }
}