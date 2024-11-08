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
    let executable_file = "temp_program"; // 临时的二进制文件名

    // 检查 C++ 文件是否存在
    if !Path::new(cpp_file).exists() {
        eprintln!("C++ source file '{}' not found!", cpp_file);
        exit(1);
    }

    // 编译 C++ 程序
    println!("Compiling {}...", cpp_file);
    let compile_output = Command::new("g++")
        .arg(cpp_file)
        .arg("-o")
        .arg(executable_file)
        .output();

    match compile_output {
        Ok(output) if output.status.success() => {
            println!("C++ program compiled successfully!");

            // 使用 time 命令来测量运行时间
            println!("Running C++ program...");
            let start_time = std::time::Instant::now();

            // 启动进程
            let mut child = Command::new("./temp_program")
                .spawn()
                .expect("Failed to start C++ program");

            // 获取进程 ID（PID）
            let pid = child.id();
            println!("C++ program running with PID: {}", pid);

            // 使用 ps 命令来获取内存使用情况
            // 这里我们等待 1 秒钟，以确保程序正在运行
            sleep(Duration::from_secs(1));

            let ps_output = Command::new("ps")
                .arg("-o")
                .arg("pid,etime,%mem,rss")
                .arg("-p")
                .arg(pid.to_string())
                .output()
                .expect("Failed to execute ps");

            // 打印内存使用情况
            let ps_output_str = String::from_utf8_lossy(&ps_output.stdout);
            println!("Memory usage info: \n{}", ps_output_str);

            // 等待 C++ 程序运行结束
            let status = child.wait().expect("C++ program didn't exit successfully");
            let elapsed_time = start_time.elapsed();

            // 打印运行时间
            if status.success() {
                println!("C++ program executed successfully.");
                println!(
                    "Program executed in {:?} seconds.",
                    elapsed_time.as_secs_f64()
                );
            } else {
                eprintln!("C++ program failed to execute.");
            }

            // 删除临时生成的二进制文件
            if let Err(e) = fs::remove_file(executable_file) {
                eprintln!("Failed to delete the executable: {}", e);
            } else {
                println!("Deleted the compiled executable '{}'.", executable_file);
            }
        }
        Ok(output) => {
            eprintln!("Compilation failed: {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute g++: {}", e);
            exit(1);
        }
    }
}
