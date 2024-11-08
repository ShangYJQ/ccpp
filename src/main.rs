use std::env;
use std::fs;
use std::process::{Command, exit};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // 获取命令行参数（C++ 源文件）
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ccpp <source_file.cpp>");
        exit(1);
    }

    let cpp_file = &args[1];
    

    // 检查 C++ 文件是否存在
    if !Path::new(cpp_file).exists() {
        eprintln!("C++ source file '{}' not found!", cpp_file);
        exit(1);
    }

    let executable_file = "tempExecutableFile"; // 临时的二进制文件名

    // 编译 C++ 程序
    println!("Compiling {}...", cpp_file);
    let compile_output = Command::new("g++")
        .arg("-Wall")
        .arg("-O2")
        .arg("-std=c++20")
        .arg(cpp_file)
        .arg("-o")
        .arg(executable_file)
        .output();

    match compile_output {
        Ok(output) if output.status.success() => {
            println!("C++ program compiled successfully!\n");
            let start_time = std::time::Instant::now();

            // 启动进程
            let mut child = Command::new("./tempExecutableFile")
                .spawn()
                .expect("Failed to start C++ program");

            
            // 等待 C++ 程序运行结束
            let status = child.wait();
            let elapsed_time = start_time.elapsed();

            // 打印运行时间
            if status.success() {
                println!("\nC++ program executed successfully.");
                println!(
                    "Program executed in {} milliseconds.",
                    elapsed_time.as_millis()
                );
            } else {
                eprintln!("\nC++ program failed to execute.");
            }

            // 删除临时生成的二进制文件
            if let Err(e) = fs::remove_file(executable_file) {
                eprintln!("Failed to delete the executable: {}", e);
            }
        }
        Ok(output) => {
            eprintln!("Compilation failed: {}\n", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute g++: {}\n", e);
            exit(1);
        }
    }
}
