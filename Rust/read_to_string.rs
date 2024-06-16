use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// 定义处理文件的函数，接收一个文件路径作为参数并返回一个 I/O 结果类型
fn process_file(path: &Path) -> io::Result<()> {
    // 打开指定路径的文件，使用 `?` 操作符处理可能的错误
    let mut file = File::open(path)?;
    // 创建一个字符串缓冲区
    let mut contents = String::new();
    // 读取整个文件到字符串缓冲区
    file.read_to_string(&mut contents)?;

    // 初始化计数器
    let mut linecount = 0;  // 行数
    let mut charcount = 0;  // 字符数
    let mut wordscount = 0; // 单词数
    let mut current_word = false;   // 标记是否在单词内部
    let mut bytescount = 0;  // 字节数

    // 按行分割字符串
    for line in contents.lines() {
        // 行数加一
        linecount += 1;

        // 计算单词数
        let mut chars_iter = line.chars();
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
        
        charcount += line.chars().count()+1;
        
        bytescount += line.len()+2;
    }

    // 输出统计结果
    println!("linecount:{}", linecount);
    println!("charcount:{}", charcount);
    println!("wordscount:{}", wordscount);
    println!("bytescount:{}",bytescount);

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