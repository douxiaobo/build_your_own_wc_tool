use std::env;
use std::io::{self,BufReader,Read};
use std::fs;

struct File {
    file_name: String,
    contents:String,
}
impl File {
    fn new(file: String,contents: String) -> Self {
        // let bytes = contents.len().to_string();
        // let lines = contents.lines().count().to_string();
        // let words = contents.split_whitespace().count().to_string();
        // let chars = contents.chars().count().to_string();
        File {
            file_name: file,
            contents:contents,
        }
    }
    fn bytes(&self) -> String {
        self.contents.len().to_string()
    }
    fn lines(&self) -> String {
        self.contents.lines().count().to_string()
    }
    fn words(&self) -> String {
        self.contents.split_whitespace().count().to_string()
    }
    fn chars(&self) -> String {
        self.contents.chars().count().to_string()
    }
}
fn main() -> Result<(), io::Error> {
    let mut command = String::new();
    let mut file_path=String::new();
    let args:Vec<String>=env::args().collect();

    let mut iter = args.iter();
    iter.next(); // skip the program name
    for arg in iter {
        if arg.starts_with("-") {
            command+=arg.get(1..).unwrap();
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
                } ,
            }
        }
    }
    // println!("Command:{}\tFile:{}", command, file_path)
    let mut contents = String::new();
    if file_path.is_empty() {
        BufReader::new(std::io::stdin().lock()).read_to_string(&mut contents)
            .unwrap_or_else(|_| {
                eprintln!("Failed to read from stdin");
                std::process::exit(1);
            });
    } else {
        let mut reader=BufReader::new(fs::File::open(file_path.clone())?);
        reader.read_to_string(&mut contents)
           .unwrap_or_else(|_| {
                eprintln!("Failed to read from file {}", file_path);
                std::process::exit(1);
            });
    }
    let file =File::new(file_path,contents);
    if command.is_empty() {
        println!("\t{}\t{}\t{}\t{}\t", file.lines(), file.words(), file.bytes(), file.file_name)
    } else {
        for ch in command.chars() {
            match ch {
                'c' => print!("\t{}", file.bytes()),
                'l' => print!("\t{}", file.lines()),
                'w' => print!("\t{}", file.words()),
                'm' => print!("\t{}", file.chars()),
                _ => {
                    eprintln!("Error: Invalid command option -{}", ch);
                    std::process::exit(1);
                }
            }
        }
        println!("{}",file.file_name);
    }
    Ok(())
}

fn is_file(path: &str) -> Result<bool, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.file_type().is_file())
}