use std::fs;
use std::io::{ stdin,Read};

fn main() {
    let mut args = std::env::args();
    args.next(); // skip the first argument, which is the name of the program

    let mut command = String::from("a");
    let file_name_or_stdin = if let Some(arg) = args.next() {
        if arg.contains(".") {
            arg.to_string()
        } else if arg == "-c" {
            command = "c".to_string();
            args.next().unwrap_or_else(|| "".to_string())
        } else if arg == "-l" {
            command = "l".to_string();
            args.next().unwrap_or_else(|| "".to_string())
        } else if arg == "-w" {
            command = "w".to_string();
            args.next().unwrap_or_else(|| "".to_string())
        } else if arg == "-m" {
            command = "m".to_string();
            args.next().unwrap_or_else(|| "".to_string())
        } else {
            println!("Invalid command");
            return;
        }
    } else {
        "".to_string()
    };

    // 根据文件名或标准输入决定读取内容的来源
    let contents = if file_name_or_stdin.is_empty() {
        // 当没有提供文件名时，从标准输入读取内容
        let mut buffer = String::new();
        // 将标准输入的内容读入buffer
        stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
        buffer
    } else {
        // 尝试从指定的文件名读取内容
        fs::read_to_string(&file_name_or_stdin).unwrap_or_else(|err| {
            // 如果读取文件失败，打印错误信息并返回空字符串
            println!("Error reading file: {}", err);
            "".to_string()
        })
    };

    println!("{} {}", result(command, contents), file_name_or_stdin);
}

fn result(command: String, contents: String) -> String {
    match command.as_str() {
        "a" => {
            return format!(
                "{}\t{}\t{}",
                contents.len(),
                contents.lines().count(),
                contents.split_whitespace().count()
            );
        }
        "c" => contents.len().to_string(),
        "l" => contents.lines().count().to_string(),
        "w" => contents.split_whitespace().count().to_string(),
        "m" => contents.chars().count().to_string(),
        _ => "Invalid command".to_string(),
    }
}