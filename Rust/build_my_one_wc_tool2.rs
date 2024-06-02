fn main() {
    let args = std::env::args().skip(1); // 直接跳过第一个参数（程序名）
    println!("{:?}", args);

    let (command, file_name) = args.fold( // 使用 fold 来累积结果
        (String::from("a"), String::new()), // 初始化 (command, file_name)
        |(mut cmd, mut name), arg| { // closure 接受累积值和当前项 arg
            if arg.contains(".") {
                name = arg; // 设置文件名为含有点号的参数
            } else if arg.starts_with("-") {
                cmd = arg[1..].to_string(); // 设置命令为去掉前缀 `-` 的参数
            } else {
                println!("Invalid command or argument: {}", arg);
                std::process::exit(1); // 遇到无效参数时退出程序
            }
            (cmd, name)
        },
    );

    println!("{:?},{:?}", command, file_name);
}