use std::fs::File;  
use std::io::{ BufReader,Read};  
use std::path::Path;  
  
fn main() {  
    let file_path = "test_file";  
    let display_path = Path::new(file_path).display();  
  
    // 打开文件  
    let file = match File::open(&file_path) {  
        Err(err) if err.kind() == ErrorKind::NotFound => {  
            eprintln!("文件 {} 未找到", display_path);  
            return;  
        },  
        Err(why) => {  
            eprintln!("无法打开文件 {}: {}", display_path, why);  
            return;  
        },  
        Ok(file) => file,  
    };  
  
    // 创建一个BufReader来优化读取操作  
    let mut reader = BufReader::new(file);  
  
    // 读取文件内容  
    let mut content = String::new();  
    match reader.read_to_string(&mut content) {  
        Err(why) => {  
            eprintln!("读取文件 {} 出错: {}", display_path, why);  
        },  
        Ok(_) => {  
            // 打印文件内容  
            println!("{}", content);  
        },  
    };  
}