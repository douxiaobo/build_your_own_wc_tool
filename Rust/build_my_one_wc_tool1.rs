fn main() {
    let mut args = std::env::args(); // 直接跳过第一个参数
    args.next(); // 跳过程序名
    println!("{:?}", args);

    let mut command = String::from("a"); // 默认命令为 `a`
    let mut file_name = String::new();

    while let Some(arg) = args.next() {
        if arg.contains("."){
            file_name = arg;
        } else if arg.starts_with("-") {
            command = arg[1..].to_string();
        } else {
            println!("Invalid command");
            return;
        }
    }
    println!("{:?},{:?}", command, file_name);
}