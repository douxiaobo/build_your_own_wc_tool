use std::env;
use std::io::{self, BufReader, BufRead};
use std::fs;

struct File {
    file_name: String,
    linecount: u32,
    charcount: u32,
    wordcount: u32,
    bytescount: u32,
}

impl File {
    fn new(file: String, reader: &mut dyn BufRead) -> Result<File, io::Error> {
        let mut linecount_tmp = 0;
        let mut charcount_tmp = 0;
        let mut wordcount_tmp = 0;
        let mut bytescount_tmp = 0;
        let mut current_word = false;

        for line_result in reader.split(b'\n') {
            let line_bytes = line_result?;
            let line_str = std::str::from_utf8(&line_bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            linecount_tmp += 1;
            bytescount_tmp += line_bytes.len() as u32 + 1;

            for ch in line_str.chars() {
                charcount_tmp += 1;
                if ch.is_whitespace() {
                    if current_word {
                        wordcount_tmp += 1; // 单词计数加一
                        current_word = false; // 重置单词标记
                    }
                } else {
                    current_word = true; // 在单词内部
                }
            }
            // 如果当前处于单词中，单词计数加一
            if current_word {
                wordcount_tmp += 1;
                current_word = false;
            }
            charcount_tmp += 1; // 换行符也算一个字符
        }

        Ok(File {
            file_name: file,
            linecount: linecount_tmp,
            charcount: charcount_tmp,
            wordcount: wordcount_tmp,
            bytescount: bytescount_tmp,
        })
    }
}

fn main() -> Result<(), io::Error> {
    let mut command = String::new();
    let mut file_path = String::new();
    let args: Vec<String> = env::args().collect();

    let mut iter = args.iter();
    iter.next(); // skip the program name
    for arg in iter {
        if arg.starts_with("-") {
            command += arg.get(1..).unwrap();
        } else {
            match is_file(arg) {
                Ok(is_file) => {
                    if is_file {
                        file_path = arg.to_string();
                    } else {
                        eprintln!("Error: {} is not a file", arg);
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                },
            }
        }
    }

    let file = if file_path.is_empty() {
        File::new(file_path.clone(), &mut BufReader::new(io::stdin()))?
    } else {
        File::new(file_path.clone(), &mut BufReader::new(fs::File::open(&file_path)?))?
    };

    if command.is_empty() {
        println!("\t{}\t{}\t{}\t{}", file.linecount, file.wordcount, file.bytescount, file.file_name)
    } else {
        for ch in command.chars() {
            match ch {
                'c' => print!("\t{}", file.bytescount),
                'l' => print!("\t{}", file.linecount),
                'w' => print!("\t{}", file.wordcount),
                'm' => print!("\t{}", file.charcount),
                _ => {
                    eprintln!("Error: Invalid command option -{}", ch);
                    std::process::exit(1);
                }
            }
        }
        println!("\t{}", file.file_name);
    }
    Ok(())
}

fn is_file(path: &str) -> Result<bool, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.file_type().is_file())
}