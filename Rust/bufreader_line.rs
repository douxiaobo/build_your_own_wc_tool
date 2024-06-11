use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

fn process_file(path: &Path) -> Result<(), std::io::Error> {
    let file = File::open(path)?; // 使用问号操作符处理错误
    let mut reader = BufReader::new(file);
    let mut linecount = 0;
    // let mut charcount = 0;
    let mut wordscount = 0;
    let mut current_word = String::new();
    // let mut bytescount = 0;
    // let mut contents = String::new();
    // let mut buffer = [0u8; 1024];

    for line_result in reader.by_ref().lines() {
        let line = line_result?;
        // 在这里处理每一行，比如计算行数、字符数或单词数
        // ...
        linecount += 1; //Okay

        for ch in line.chars() {
            //Okay
            if ch.is_whitespace() {
                if !current_word.is_empty() {
                    wordscount += 1;
                    current_word.clear();
                }
            } else {
                current_word.push(ch);
            }
        }
        if !current_word.is_empty() {
            wordscount += 1;
            current_word.clear();
        }

        // for _ in line.bytes() {
        //     bytescount += 1;
        // }

        // bytescount += line.as_bytes().len();     //Wrong

        // charcount += line.chars().count();       //Wrong
        // contents += &(line + "\n");
    }

    let file = File::open(path)?; //OK for outputs the number of bytes in a file.
    let metadata = file.metadata()?;
    let bytescount = metadata.len();

    let file = File::open(path)?; //Ok for output the number of characters in a file.
    reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);
    let charcount = contents.chars().count();

    println!("linecount:{linecount}"); //7145,OK

    println!("charcount:{charcount}"); //339292     //difficlut within lines

    println!("wordscount:{wordscount}"); //58164,OK

    println!("bytescount:{bytescount}"); //342190   //difficult within lines

    // println!("contents:{contents}");     //OK

    Ok(())
}

fn main() {
    let path = Path::new("test.txt");
    if let Err(e) = process_file(path) {
        println!("Error: {}", e);
    }
}
