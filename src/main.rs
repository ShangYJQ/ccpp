use std::process::{Command, exit};
use std::fs;

fn main() {
    // 编译命令的结果处理
    match compile_output {
        Ok(output) if output.status.success() => {
            println!("C++ program compiled successfully!\n");
            let start_time = std::time::Instant::now();

            // 启动进程
            let mut child = Command::new("./tempExecutableFile")
                .spawn()
                .expect("Failed to start C++ program");

            // 等待 C++ 程序运行结束
            let status = child.wait().expect("Failed to wait on child process");
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
            if let Err(e) = fs::remove_file("tempExecutableFile") {
                eprintln!("Failed to delete the executable: {}", e);
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
