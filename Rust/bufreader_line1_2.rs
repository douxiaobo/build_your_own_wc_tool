// 引入所需的库和模块
use std::fs::File;
use std::io::{self, BufRead, BufReader};    // io 包含了标准输入输出错误处理
use std::path::Path;    // 提供了路径相关操作

// Okay
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
        // 尝试将字节数据转换为字符串，若失败则转换错误并返回
        let line_str = match String::from_utf8(line_bytes.to_vec()) {
            Ok(s) => s,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };

        // 行数加一
        linecount += 1;

        // Adjust for potential "\r\n"
        // 遍历字符串中的字符以统计字符数和单词数
        let mut chars_iter = line_str.chars();
        let prev_char = chars_iter.next_back(); // 获取行尾字符（如果存在）
        // println!("prev_char: {:?}", prev_char); // 打印行尾字符，都是'\r'
        for ch in chars_iter {
            // 遇到空格或换行符，说明当前单词结束
            if ch.is_whitespace() {
                if current_word {
                    wordscount += 1;    // 单词计数加一
                    current_word = false;   // 重置单词标记
                }
            } else {
                current_word = true;    // 在单词内部
            }
            charcount += 1; // 总字符数加一
        }

        // 如果行以 "\r\n" 结束，调整字符计数
        if prev_char.map_or(false, |c| c == '\r') {
            charcount += 1;
        }
        // 如果当前处于单词中，单词计数加一
        if current_word {
            wordscount += 1;
            current_word = false;
        }

        // Counting bytes, adjusting for "\r\n"
        // 计算字节总数，考虑 "\r\n" 的情况
        bytescount += line_bytes.len();
        // 若行尾有 "\r"，调整字节和字符计数
        if prev_char == Some('\r') {
            charcount+=1;
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