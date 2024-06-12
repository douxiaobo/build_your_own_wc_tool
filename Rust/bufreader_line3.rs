use std::fs::File;  
use std::io::{BufRead, BufReader};  
use std::path::Path;  
  
// it is Wrong code from Baidu Yiyan(AI language model)

fn process_file(path: &Path) -> Result<(), std::io::Error> {  
    let file = File::open(path)?;  
    let reader = BufReader::new(file);  
    let mut linecount = 0;  
    let mut charcount = 0;  
    let mut wordscount = 0;  
    let mut in_word = false;  
    let mut bytescount = 0;  
  
    for line_result in reader.lines() {  
        let line = line_result?;  
        linecount += 1;  
  
        let mut chars = line.chars();  
        while let Some(ch) = chars.next() {  
            charcount += 1; // 直接累加字符数  
  
            if ch.is_whitespace() {  
                if in_word {  
                    wordscount += 1;  
                    in_word = false;  
                }  
            } else {  
                if !in_word {  
                    in_word = true;  
                }  
            }  
        }  
  
        // 如果最后一个字符不是空格，那么也算一个单词  
        if in_word {  
            wordscount += 1;  
        }  
  
        bytescount += line.len(); // 使用len_utf8来获取UTF-8编码的字符的字节数  
    }  
  
    println!("linecount:{}", linecount);  
    println!("charcount:{}", charcount);  
    println!("wordscount:{}", wordscount);  
    println!("bytescount:{}", bytescount);  
  
    Ok(())  
}  
  
fn main() {  
    let path = Path::new("test.txt");  
    if let Err(e) = process_file(path) {  
        println!("Error: {}", e);  
    }  
}