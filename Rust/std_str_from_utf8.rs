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
    let mut bytescount = 0; // 字节总数

    // 使用 `split` 方法按行读取文件内容，其中 `b'\n'` 是换行符的字节表示
    for line_result in reader.split(b'\n') {
        // 解包每一行的字节数据，错误则立即返回
        let line_bytes = line_result?;

        // 使用 `std::str::from_utf8` 尝试将字节数据转换为字符串切片
        let line_str = std::str::from_utf8(&line_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // 行数加一
        linecount += 1;

        // 计算单词数和字符数
        let mut chars_iter = line_str.chars();
        let prev_char = chars_iter.next_back(); // 获取行尾字符（如果存在）
        for ch in chars_iter.by_ref() {
            if ch.is_whitespace() {
                if current_word {
                    wordscount += 1;    // 单词计数加一
                    current_word = false;   // 重置单词标记
                }
            } else {
                current_word = true;    // 在单词内部
            }
            // charcount += 1; // 总字符数加一
        }

        // 如果当前处于单词中，单词计数加一
        if current_word {
            wordscount += 1;
            current_word = false;
        }

        charcount += line_str.chars().count();
        if prev_char==Some('\r'){
            charcount+=1;   // 如果行尾是'\r'，则多计一个字符
        }

        // 计算字节总数，考虑 "\r\n" 的情况
        bytescount += line_bytes.len();
        if prev_char == Some('\r') {
            bytescount += 1; 
        }
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
    let path = Path::new("test.txt");
    // 调用 process_file 函数处理文件，打印任何发生的错误
    if let Err(e) = process_file(&path) {
        println!("Error: {}", e);
    }
}

// 提到的 to_vec 和 String::from_utf8 确实会创建新的字符串副本，这在处理大文件时可能会影响性能。
// 使用 std::str::from_utf8 可以避免这种情况，因为它尝试将原始字节切片转换为字符串切片，而不会分配新的内存。

// douxiaobo@192 Rust % rustc std_str_from_utf8.rs
// douxiaobo@192 Rust % ./std_str_from_utf8       
// linecount:7145
// charcount:339292
// wordscount:58164
// bytescount:342190
