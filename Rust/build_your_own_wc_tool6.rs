use std::env;
use std::io::{self,BufReader,BufRead};
use std::fs;

struct File {
    file_name: String,
    linecount:u32,
    charcount:u32,
    wordcount:u32,
    bytescount:u32,
}
impl File {
    fn new(file: String,reader: &mut dyn BufRead)->Result<File,io::Error> {
        let mut linecount = 0;
        let mut charcount = 0;
        let mut wordcount = 0;
        let mut bytescount = 0;

        for line_result in reader.lines() { // 使用lines()迭代器简化
            let line = line_result?;
            
            linecount += 1;
            charcount += line.chars().count() as u32 +2; // 这里不需要+1
            
            // 利用split_whitespace优化单词计数
            wordcount += line.split_whitespace().count() as u32;
            
            // 字节计数已包含实际行内容中的换行符（如果存在）
            bytescount += line.as_bytes().len() as u32 +2;
            // 不需要对换行符额外+1，它已经在line.as_bytes().len()中被计算
        }
        Ok(File {
            file_name: file,
            linecount,
            charcount,
            wordcount,
            bytescount,
        })        
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
    let file = if file_path.is_empty() {
        File::new(file_path.clone(), &mut BufReader::new(io::stdin()))?
    } else {
        File::new(file_path.clone(), &mut BufReader::new(fs::File::open(&file_path)?))?
    };

    if command.is_empty() {
        println!("\t{}\t{}\t{}\t{}\t", file.linecount, file.wordcount, file.bytescount, file.file_name)
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
        println!("/t{}",file.file_name);
    }
    Ok(())
}

fn is_file(path: &str) -> Result<bool, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.file_type().is_file())
}

// douxiaobo@192 Rust % rustc build_your_own_wc_tool3.rs
// douxiaobo@192 Rust % ./build_your_own_wc_tool3 ./test.txt
// 	7145	58164	342190	./test.txt	
// douxiaobo@192 Rust % ./build_your_own_wc_tool3 ./test.txt -clwm
// 	342190	7145	58164	339292./test.txt
// douxiaobo@192 Rust % cat ./test.txt | .build_your_own_wc_tool3 
// zsh: command not found: .build_your_own_wc_tool3
// douxiaobo@192 Rust % cat ./test.txt | ./build_your_own_wc_tool3
// 	7145	58164	342190		
// douxiaobo@192 Rust % 
