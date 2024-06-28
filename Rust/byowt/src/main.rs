use std::env;
use tokio::io::{self, AsyncBufReadExt};
use tokio::fs;

struct File {
    file_name: String,
    linecount: u32,
    charcount: u32,
    wordcount: u32,
    bytescount: u32,
}

impl File {
    async fn new(file_name: String, reader: &mut (impl AsyncBufReadExt + Unpin)) -> Result<File, io::Error> {
        let mut linecount_tmp = 0;
        let mut charcount_tmp = 0;
        let mut wordcount_tmp = 0;
        let mut bytescount_tmp = 0;
        let mut current_word = false;

        let mut contents = String::new();
        while let Ok(n) = reader.read_line(&mut contents).await {
            if n == 0 {
                break;
            }
            bytescount_tmp += n as u32;
            charcount_tmp += contents.chars().count() as u32;
            
            linecount_tmp += 1;

            for ch in contents.chars() {
                if ch.is_whitespace() {
                    if current_word {
                        wordcount_tmp += 1;
                        current_word = false;
                    }
                } else {
                    current_word = true;
                }
            }
            if current_word {
                wordcount_tmp += 1;
                current_word = false;
            }
            contents.clear();
        }

        Ok(File {
            file_name,
            linecount: linecount_tmp,
            charcount: charcount_tmp,
            wordcount: wordcount_tmp,
            bytescount: bytescount_tmp,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut command = String::new();
    let mut file_path = String::new();
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        if arg.starts_with("-") {
            command.push_str(&arg[1..]);
        } else {
            match fs::metadata(arg).await {
                Ok(metadata) if metadata.is_file() => file_path = arg.clone(),
                Ok(_) => {
                    eprintln!("Error: {} is not a file", arg);
                    std::process::exit(1);
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                },
            }
        }
    }

    // let file = match file_path.is_empty() {
    //     true => File::new("<stdin>".into(), &mut io::BufReader::new(io::stdin())).await?,
    //     false => {
    //         let file = fs::File::open(&file_path).await?;
    //         File::new(file_path.clone(), &mut io::BufReader::new(file)).await?
    //     },
    // };

    let file = match file_path.is_empty() {
        true => {
            let mut stdin_reader = io::BufReader::new(io::stdin());
            File::new("<stdin>".into(), &mut stdin_reader).await?
        },
        false => {
            let file = fs::File::open(&file_path).await?;
            let mut file_reader = io::BufReader::new(file);
            File::new(file_path.clone(), &mut file_reader).await?
        },
    };

    if command.is_empty() {
        println!("\t{}\t{}\t{}\t{}", file.linecount, file.wordcount, file.bytescount, file.file_name)
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
        println!("\t{}", file.file_name);
    }
    Ok(())
}

// douxiaobo@192 byowt % cargo run ../test.txt -clwm
//    Compiling byowt v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Rust/byowt)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
//      Running `target/debug/byowt ../test.txt -clwm`
// 	342190	7145	58164	339292	../test.txt
// douxiaobo@192 byowt % cat ../test.txt | cargo run -clwm
// error: unexpected argument '-c' found

//   tip: a similar argument exists: '--config'

// Usage: cargo run [OPTIONS] [ARGS]...

// For more information, try '--help'.
// douxiaobo@192 byowt % cargo build
//    Compiling byowt v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Rust/byowt)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
// douxiaobo@192 byowt % cat ../test.txt | ./target/debug/byowt -clwm
// 	0	0	0	0	./target/debug/byowt
// douxiaobo@192 byowt % 

// douxiaobo@192 byowt % cargo build
//    Compiling byowt v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Rust/byowt)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
// douxiaobo@192 byowt % ./target/debug/byowt ../test.txt -clwm
// 	342190	7145	58164	339292	../test.txt
// douxiaobo@192 byowt % cat ../test.txt | ./target/debug/byowt -clwm
// 	0	0	0	0	./target/debug/byowt
// douxiaobo@192 byowt % 

