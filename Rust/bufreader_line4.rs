use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// it is wrong from Kimi.ai

fn process_file(path: &Path) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut linecount = 0;
    let mut charcount = 0;
    let mut wordscount = 0;
    let mut current_word = false;
    let mut bytescount = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        linecount += 1;
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            charcount += ch.len_utf8(); // 计算每个字符的实际字节长度
            if ch.is_whitespace() {
                if current_word {
                    wordscount += 1;
                    current_word = false;
                }
            } else {
                current_word = true;
            }
        }
        if current_word {
            wordscount += 1;
            current_word = false;
        }
        bytescount += line.len(); // 保持原样，统计字节数
    }

    println!("linecount:{}", linecount);
    println!("charcount:{}", charcount);        //wrong
    println!("wordscount:{}", wordscount);
    println!("bytescount:{}", bytescount);      //wrong

    Ok(())
}

fn main() {
    let path = Path::new("test.txt");
    if let Err(e) = process_file(path) {
        println!("Error: {}", e);
    }
}