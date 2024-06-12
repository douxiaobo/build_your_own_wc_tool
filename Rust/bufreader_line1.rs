use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn process_file(path: &Path) -> Result<(), std::io::Error> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut linecount = 0;
    let mut charcount = 0;
    let mut wordscount = 0;
    let mut current_word = false;
    let mut bytescount = 0;

    let lines_iters = reader.lines().map(|l| l.expect("Failed to read line"));
    for lines_iter in lines_iters {
        linecount += 1; //OK
        for ch in lines_iter.chars() {
            //OK
            if ch.is_whitespace() {
                if current_word == true {
                    wordscount += 1;
                    current_word = false;
                }
            } else {
                current_word = true;
            }
        }
        if current_word == true {
            wordscount += 1;
            current_word = false;
        }
        charcount += lines_iter.chars().count(); //325002，Wrong
        bytescount += lines_iter.as_bytes().len(); //327900，Wrong
    }

    println!("linecount:{linecount}"); //7145,OK
    println!("charcount:{charcount}"); //339292     //difficlut within lines
    println!("wordscount:{wordscount}"); //58164,OK
    println!("bytescount:{bytescount}"); //342190   //difficult within lines

    Ok(())
}

fn main() {
    let path = Path::new("test.txt");
    if let Err(e) = process_file(path) {
        println!("Error: {}", e);
    }
}
