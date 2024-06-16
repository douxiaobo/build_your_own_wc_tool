// 引入所需的库和模块
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// 定义处理文件的函数，接收一个文件路径作为参数并返回一个 I/O 结果类型
fn process_file(path: &Path) -> io::Result<()> {
    // 打开指定路径的文件，使用 `?` 操作符处理可能的错误
    let file = File::open(path)?;
    // 创建一个缓冲读取器来提高文件读取效率
    let reader = BufReader::new(file);

    // 初始化计数器
    let mut linecount = 0;  // 行数
    let mut charcount = 0;  // 字符数
    let mut wordscount = 0; // 单词数
    let mut current_word = false;   // 标记是否在单词内部
    let mut bytescount = 0;  // 字节数

    // 使用 `split` 方法按行读取文件内容，这里使用 `\n` 作为分隔符
    for line_result in reader.split(b'\n') {
        // 解包每一行的字节数据，错误则立即返回
        let line_bytes = line_result?;
        // 尝试将字节数据转换为字符串，若失败则转换错误并返回
        let line_str = std::str::from_utf8(&line_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // 行数加一
        linecount += 1;

        // 计算单词数
        let mut chars_iter = line_str.chars();
        for ch in chars_iter.by_ref() {
            if ch.is_whitespace() {
                if current_word {
                    wordscount += 1;    // 单词计数加一
                    current_word = false;   // 重置单词标记
                }
            } else {
                current_word = true;    // 在单词内部
            }
        }
        // 如果当前处于单词中，单词计数加一
        if current_word {
            wordscount += 1;
            current_word = false;
        }

        charcount += line_str.chars().count()+1;

        // 计算字节总数，考虑 "\r\n" 的情况
        bytescount += line_bytes.len()+1;
    }

    // 输出统计结果
    println!("linecount:{}", linecount);
    println!("charcount:{}", charcount);
    println!("wordscount:{}", wordscount);
    println!("bytescount:{}", bytescount);

    // 函数成功完成，返回 Ok
    Ok(())
}

// 主函数
fn main() {
    // 设置文件路径
    let path = Path::new("test.txt"); // 这里可以根据需要更改文件名
    // 调用 process_file 函数处理文件，打印任何发生的错误
    if let Err(e) = process_file(&path) {
        println!("Error: {}", e);
    }
}

// linecount:7145
// charcount:332147
// wordscount:58164
// bytescount:342190



// douxiaobo@192 Rust % rustup default nightly
// info: syncing channel updates for 'nightly-aarch64-apple-darwin'
// info: latest update on 2024-06-16, rust version 1.81.0-nightly (3cf924b93 2024-06-15)
// info: downloading component 'cargo'
// info: downloading component 'clippy'
// info: downloading component 'rust-docs'
//  15.7 MiB /  15.7 MiB (100 %)   7.4 MiB/s in  2s ETA:  0s
// info: downloading component 'rust-std'
//  24.8 MiB /  24.8 MiB (100 %)   5.5 MiB/s in  4s ETA:  0s
// info: downloading component 'rustc'
//  51.6 MiB /  51.6 MiB (100 %)   5.7 MiB/s in  8s ETA:  0s
// info: downloading component 'rustfmt'
// info: installing component 'cargo'
// info: installing component 'clippy'
// info: installing component 'rust-docs'
//  15.7 MiB /  15.7 MiB (100 %)   4.3 MiB/s in  3s ETA:  0s
// info: installing component 'rust-std'
//  24.8 MiB /  24.8 MiB (100 %)  24.2 MiB/s in  1s ETA:  0s
// info: installing component 'rustc'
//  51.6 MiB /  51.6 MiB (100 %)  25.3 MiB/s in  1s ETA:  0s
// info: installing component 'rustfmt'
// info: default toolchain set to 'nightly-aarch64-apple-darwin'

//   nightly-aarch64-apple-darwin installed - rustc 1.81.0-nightly (3cf924b93 2024-06-15)

// douxiaobo@192 Rust % cargo new benches_split
//     Creating binary (application) `benches_split` package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
