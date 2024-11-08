use colored::*; // 引入 colored 库来处理彩色输出
use std::env;
use std::env::consts::OS;
use std::fs;
use std::process::{exit, Command}; // 引入 OS 常量

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查命令行参数是否正确
    if args.len() != 2 {
        eprintln!("Usage: ccpp <source_file.cpp>\n");
        exit(1);
    }

    // 获取输入的 C++ 文件名
    let cpp_file = &args[1];

    // 检查 C++ 文件是否存在
    if fs::metadata(cpp_file).is_err() {
        eprintln!("Error: C++ source file '{}' not found.\n", cpp_file);
        exit(1);
    }

    // 1. 调用 g++ 编译 C++ 源代码文件
    let compile_result = Command::new("g++")
        .arg("-Wall")
        .arg("-O2")
        .arg("-std=c++20")
        .arg(cpp_file) // 请根据需要替换成你实际的 C++ 源文件名
        .arg("-o")
        .arg("tempExecutableFile") // 指定输出可执行文件的名称
        .output();

    // 2. 检查编译的结果
    let compile_output = match compile_result {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute g++: {}\n", e);
            exit(1);
        }
    };

    // 3. 检查编译是否成功
    if compile_output.status.success() {
        println!("C++ program compiled successfully!\n");
        let start_time = std::time::Instant::now();

        // 启动编译后的 C++ 可执行文件
        let mut child = Command::new("./tempExecutableFile")
            .spawn()
            .expect("Failed to start program\n");

        // 等待 C++ 程序运行结束
        let status = child.wait().expect("Failed to wait on child process\n");
        let elapsed_time = start_time.elapsed();

        // 打印运行时间
        if status.success() {
            println!("\nC++ program executed successfully.");
            // 引入 colored 库来处理彩色输出

            let elapsed_time_str = if OS == "windows" {
                // Windows 不使用颜色
                format!("{} ms", elapsed_time.as_millis())
            } else {
                // 其他平台应用绿色
                format!("{} ms", elapsed_time.as_millis()).green()
            };

            println!("Running time: {}.", elapsed_time_str); // 这里不再多一个括号
        } else {
            eprintln!("Program failed to execute.\n");
        }

        // 删除临时生成的二进制文件

        let del_file = if OS == "windows" {
            "tempExecutableFile.exe"
        } else {
            "tempExecutableFile"
        };

        if let Err(e) = fs::remove_file(del_file) {
            eprintln!("Failed to delete the executable: {}\n", e);
        }
    } else {
        eprintln!(
            "Compilation failed: {}\n",
            String::from_utf8_lossy(&compile_output.stderr)
        );
        exit(1);
    }
}
