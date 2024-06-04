use std::env;
use std::io::{BufReader,Read};
use std::fs::File;

fn main(){
    let mut command=String::new();
    let mut file=String::new();
    let args:Vec<String>=env::args().collect(); 

    // for arg in args.iter(){
    //     println!("{:?}",arg);
    // }
    // douxiaobo@192 Rust % ./build_your_own_wc_tool1 test.txt 
    // "./build_your_own_wc_tool1"
    // "test.txt"
    
    // Iterate over the iterator instead of calling next() on the vector
    let mut iter = args.iter();
    let _ = iter.next(); // Skip the first argument (the program name)
    for arg in iter{
        match arg {
            arg if arg.starts_with("-") => {
                command+=arg.get(1..).unwrap();
            }
            arg if arg.contains(".")=>{
                file=arg.to_string();
            }
            _=>{
                eprintln!("Usage: {} [-c|-l|-w|-m] [file]",args[0]);
                std::process::exit(1);
            }
        }
    }

    // println!("Command: {}\nFile:{}",command,file);

    if command.is_empty(){
        command="lwc".to_string();
    }

    let mut contents=String::new();

    if file.is_empty(){
        BufReader::new(std::io::stdin().lock()).read_to_string(&mut contents)
            .unwrap_or_else(|_| {
                eprintln!("Failed to read from stdin");
                std::process::exit(1);
            });
    } else {
        match File::open(&file) {
            Ok(f) => BufReader::new(f).read_to_string(&mut contents)
                .unwrap_or_else(|_| {
                    eprintln!("Failed to read from file: {}", file);
                    std::process::exit(1);
                }),
            Err(_) => {
                eprintln!("Error: No such file or directory: {}", file);
                std::process::exit(1);
            }
        };
        // BufReader::new(File::open(file.clone()).unwrap())
        //     .read_to_string(&mut contents)
        //     .unwrap_or_else(|_| {
        //         eprintln!("Failed to read from file: {}", file);
        //         std::process::exit(1);
        //     });
    }

    let result = result(command, contents, file);
    if !result.is_empty() {
        println!("{}", result);
    }
}

fn result(command: String, contents: String, file: String) -> String {
    let mut result = String::new();
    result+="\t";
    let should_output=!contents.is_empty();
    for ch in command.chars() {
        match ch {
            'c' =>{
                result+=&(contents.len().to_string()+"\t") ;                
            },
            'l' =>{
                result+=&(contents.lines().count().to_string()+"\t") ;
            },
            'w' =>{
                result+=&(contents.split_whitespace().count().to_string()+"\t") ;
            },
            'm' =>{
                result+=&(contents.chars().count().to_string()+"\t") ;
            },
            _ => return String::from("Invalid command"),
        }
    }
    if should_output{
            result+&file
    } else {
        String::from("")
    }

}