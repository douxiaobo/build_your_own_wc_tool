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
        // let mut linecount_tmp=0;
        // let mut charcount_tmp=0;
        // let mut wordcount_tmp=0;
        // let mut bytescount_tmp=0;
        // let mut current_word = false;

        let mut linecount = 0;
        let mut charcount = 0;
        let mut wordcount = 0;
        let mut bytescount = 0;

        for line_result in reader.split(b'\n') {
            let line_bytes = line_result?;
            let line_str = std::str::from_utf8(&line_bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            // Process the line to count characters, words, and bytes
            (linecount, charcount, wordcount, bytescount) = process_line(line_str, line_bytes.len() as u32, linecount, charcount, wordcount, bytescount);
            
            // Add newline byte count
            charcount+=1;
            bytescount += 1;
            
            // linecount_tmp+=1;
            
            // charcount_tmp+=line_str.bytes().count() as u32;

            // let mut chars_iter = line_str.chars();
            // for ch in chars_iter.by_ref() {
            //     if ch.is_whitespace() {
            //         if current_word {
            //             wordcount_tmp += 1;    // 单词计数加一
            //             current_word = false;   // 重置单词标记
            //         }
            //     } else {
            //         current_word = true;    // 在单词内部
            //     }
            // }
            // // 如果当前处于单词中，单词计数加一
            // if current_word {
            //     wordcount_tmp += 1;
            //     current_word = false;
            // }
            
            // bytescount_tmp+=line_bytes.len() as u32+1;
        }
        // Ok(File {
        //     file_name: file,
        //     linecount:linecount_tmp,
        //     charcount:charcount_tmp,
        //     wordcount:wordcount_tmp,
        //     bytescount:bytescount_tmp,
        // })        
        Ok(File {
            file_name: file,
            linecount,
            charcount,
            wordcount,
            bytescount,
        })
    }
}

// Helper function to process a line and update counts
fn process_line(line: &str, line_byte_len: u32, mut linecount: u32, mut charcount: u32, mut wordcount: u32, mut bytescount: u32) -> (u32, u32, u32, u32) {
    linecount += 1;
    charcount += line.chars().count() as u32;
    bytescount += line_byte_len;

    let mut current_word = false;
    for ch in line.chars() {
        if ch.is_whitespace() {
            if current_word {
                wordcount += 1;
                current_word = false;
            }
        } else {
            current_word = true;
        }
    }
    // If the line ends with a non-whitespace character, increment word count
    if current_word {
        wordcount += 1;
    }

    (linecount, charcount, wordcount, bytescount)
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
        println!("\t{}",file.file_name);
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
