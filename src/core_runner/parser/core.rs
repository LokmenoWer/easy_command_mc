use std::io::{self, Write};
use std::process::Command;
use crate::core_runner::config;

fn print_logo(){
    println!(r"
     _   _ ________  ___
    | | | /  ___|  \/  |
    | |_| \ `--.| .  . | __ _ _ __   __ _  ___ _ __
    |  _  |`--. \ |\/| |/ _` | '_ \ / _` |/ _ \ '__|
    | | | /\__/ / |  | | (_| | | | | (_| |  __/ |
    \_| |_|____/\_|  |_/\__,_|_| |_|\__, |\___|_|
                                     __/ |
                                    |___/           ");
}

//获取自身版本号，并组成版本信息打印
pub fn print_version(){
    let version = env!("CARGO_PKG_VERSION");
    println!("Version: {}", version);
}

//获取自身作者，并组成作者信息打印
fn print_author(){
    let author = env!("CARGO_PKG_AUTHORS");
    println!("Author: {}", author);
}

//进行简要打印logo和版本信息
pub fn print_short(){
    print_logo();
    print_version();
    print_author();
}


//使用一个方法解析用户输入
pub fn get_input(){
    
    loop {
        //在用户输入前添加一个提示符
        print!("> ");
        //使用flush()方法清空缓冲区
        io::stdout().flush()
            .expect("Failed to flush stdout");
        
        //定义一个可变的字符串变量，用于存储用户输入
        let mut input = String::new();
        //使用read()方法读取用户输入
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        //使用trim()方法去除用户输入的换行符
        let input = input.trim();
        parser_command(input);

    }
}


fn parser_command(input: &str){
    match input {
        "help" => output_help(),
        "version" => output_version(),
        "author" => print_author(),
        "exit" => output_exit(),
        "clear" => output_clear(),
        "server-edit" => edit_server_config(),
        _ => println!("error"),
    }
}

fn edit_server_config(){
    //编辑服务器配置文件
    println!("编辑服务器配置文件");
    //从本地读取配置文件
    let mut config = config::Config::load();
    //使用迭代器遍历服务器配置文件
    for server in config.servers.iter() {
        println!("服务器名称：{}", server.name);
    }
    loop{
        println!("请输入要编辑的服务器名称");
        print!("> ");
        //使用flush()方法清空缓冲区
        io::stdout().flush()
            .expect("Failed to flush stdout");
        //获取用户输入
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        //找出用户输入的服务器名称所对应的服务器
        let mut server = config.servers.iter_mut().find(|s| s.name == input);
        //判断是否找到了服务器
        if server.is_none() {
            println!("未找到服务器");
            continue;
        }
        //获取服务器
        let server = server.unwrap();
        //打印服务器信息
        println!("服务器名称：{}", server.name);
        println!("服务器版本：{}", server.version);
        println!("服务器路径：{}", server.path);
        println!("服务器启动参数：{}", server.args);
        

    }
    

    



}


//终止程序
fn output_exit(){
    std::process::exit(0);
}


fn output_version(){
    //打印版本信息
    print_version();
}



fn output_clear(){
    //清空所有输出内容并且重新打印logo和版本信息
    print!("\x1B[2J\x1B[1;1H");
    //判断运行环境，如果是windows则使用cls命令清空，如果是linux则使用clear命令清空
    if cfg!(target_os = "windows") {
        //windows
        //使用system()方法执行命令
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to execute command");
    } else {
        //linux
        //使用system()方法执行命令
        std::process::Command::new("sh")
            .arg("-c")
            .arg("clear")
            .status()
            .expect("Failed to execute command");
    }

    print_logo();
    print_version();
    print_author();

}


fn output_help(){
    println!(r"帮助文本:
    help: 显示帮助文本
    version: 显示版本信息
    server-edit: 编辑服务器配置文件
    server-add: 添加服务器
    server-remove: 删除服务器
    server-list: 显示服务器列表
    server-start: 启动服务器
    server-stop: 停止服务器
    server-restart: 重启服务器
    server-status: 显示服务器状态
    server-log: 显示服务器日志
    server-backup: 备份服务器
    server-restore: 恢复服务器
    ");
}