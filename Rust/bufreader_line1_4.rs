use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn process_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut linecount = 0;
    let mut charcount = 0;
    let mut wordscount = 0;
    let mut current_word = false;
    let mut bytescount = 0;

    for line_result in reader.lines() { // Use `lines()` to get each line as a String
        let line = line_result?; // This already handles UTF-8 conversion and newline characters

        linecount += 1;
        
        // Process the line without converting from/to Vec<u8>
        let mut chars_iter = line.chars();
        let prev_char = chars_iter.next_back();

        for ch in chars_iter.by_ref() {
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

        charcount += line.chars().count();
        if let Some('\r') = prev_char {
            charcount += 2;
        }

        bytescount += line.as_bytes().len();
        if prev_char == Some('\r') {
            bytescount += 1;
        }
    }

    println!("linecount:{}", linecount);
    println!("charcount:{}", charcount);
    println!("wordscount:{}", wordscount);
    println!("bytescount:{}", bytescount);

    Ok(())
}

fn main() {
    let path = Path::new("test.txt");
    if let Err(e) = process_file(&path) {
        println!("Error: {}", e);
    }
}

// linecount:7145
// charcount:325002     //wrong
// wordscount:58040
// bytescount:327900
