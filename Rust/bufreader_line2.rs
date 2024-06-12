use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// it is Wrong code from Alibaba TongyilingmaYiyan(AI language model)

fn process_file(path: &Path) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut linecount = 0;
    let mut charcount = 0;
    let mut wordscount = 0;
    let mut current_word = false;
    let mut bytescount = 0;

    for line_result in reader.lines() { // 直接迭代 reader.lines() 的结果
        let line = line_result?;
        linecount += 1;
        
        for ch in line.chars() {
            charcount += 1; // 在这里累加字符计数
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
        bytescount += line.as_bytes().len(); // 使用 as_bytes() 来计算字节长度
    }

    println!("linecount:{}", linecount);
    println!("charcount:{}", charcount);        // wrong
    println!("wordscount:{}", wordscount);
    println!("bytescount:{}", bytescount);      // wrong

    Ok(())
}

fn main() {
    let path = Path::new("test.txt");
    if let Err(e) = process_file(path) {
        println!("Error: {}", e);
    }
}