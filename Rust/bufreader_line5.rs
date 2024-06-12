use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// It is wrong in Google Gemini.

fn process_file(path: &Path) -> Result<(), std::io::Error> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut linecount = 0;
    let mut charcount = 0;
    let mut wordscount = 0;
    let mut current_word = false;
    let mut bytescount = 0;

    for line in reader.lines().map(|l| l.expect("Failed to read line")) {
        linecount += 1;

        for ch in line.chars() {
            charcount += 1;

            if ch.is_whitespace() {
                if current_word == true {
                    wordscount += 1;
                    current_word = false;
                }
            } else {
                current_word = true;
            }
        }

        // Check for the last word in the line
        if current_word == true {
            wordscount += 1;
            current_word = false;
        }

        bytescount += line.bytes().count(); // Count bytes in the line
    }

    println!("linecount:{linecount}");
    println!("charcount:{charcount}");      //wrong
    println!("wordscount:{wordscount}");
    println!("bytescount:{bytescount}");    //wrong

    Ok(())
}

fn main() {
    let path = Path::new("test.txt");
    if let Err(e) = process_file(path) {
        println!("Error: {}", e);
    }
}