use crate::ProcessManager;

pub fn run_menu(pm: &mut ProcessManager) {
    loop {
        println!("\n╔══════════════════════════════════════════════════════╗");
        println!("║                    主菜单                            ║");
        println!("╠══════════════════════════════════════════════════════╣");
        println!("║  1. 创建进程                                         ║");
        println!("║  2. 撤销进程                                         ║");
        println!("║  3. 时间片到                                         ║");
        println!("║  4. 挂起进程                                         ║");
        println!("║  5. 激活进程                                         ║");
        println!("║  6. 执行一次调度周期                                  ║");
        println!("║  7. 显示系统状态                                      ║");
        println!("║  8. 生成快照                                         ║");
        println!("║  9. 批量创建测试进程                                  ║");
        println!("║  s. 显示调度器统计信息                                ║");
        println!("║  0. 退出                                             ║");
        println!("╚══════════════════════════════════════════════════════╝");
        print!("请选择操作 (0-9): ");

        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let choice = input.trim();

        match choice {
            "1" => {
                print!("请输入优先级 (0-10): ");
                io::stdout().flush().unwrap();
                let mut priority_input = String::new();
                io::stdin().read_line(&mut priority_input).unwrap();
                
                match priority_input.trim().parse::<u32>() {
                    Ok(priority) => {
                        match pm.create_process(priority) {
                            Ok(pid) => println!("✓ 进程 {} 创建成功", pid),
                            Err(e) => println!("✗ 错误: {}", e),
                        }
                    }
                    Err(_) => println!("✗ 无效的优先级"),
                }
            }
            "2" => {
                print!("请输入要撤销的进程PID: ");
                io::stdout().flush().unwrap();
                let mut pid_input = String::new();
                io::stdin().read_line(&mut pid_input).unwrap();
                
                match pid_input.trim().parse::<u32>() {
                    Ok(pid) => {
                        match pm.terminate_process(pid) {
                            Ok(_) => {},
                            Err(e) => println!("✗ 错误: {}", e),
                        }
                    }
                    Err(_) => println!("✗ 无效的PID"),
                }
            }
            "3" => {
                match pm.time_slice_expired() {
                    Ok(_) => {},
                    Err(e) => println!("✗ 错误: {}", e),
                }
            }
            "4" => {
                print!("请输入要挂起的进程PID: ");
                io::stdout().flush().unwrap();
                let mut pid_input = String::new();
                io::stdin().read_line(&mut pid_input).unwrap();
                
                match pid_input.trim().parse::<u32>() {
                    Ok(pid) => {
                        match pm.suspend_process(pid) {
                            Ok(_) => {},
                            Err(e) => println!("✗ 错误: {}", e),
                        }
                    }
                    Err(_) => println!("✗ 无效的PID"),
                }
            }
            "5" => {
                print!("请输入要激活的进程PID: ");
                io::stdout().flush().unwrap();
                let mut pid_input = String::new();
                io::stdin().read_line(&mut pid_input).unwrap();
                
                match pid_input.trim().parse::<u32>() {
                    Ok(pid) => {
                        match pm.activate_process(pid) {
                            Ok(_) => {},
                            Err(e) => println!("✗ 错误: {}", e),
                        }
                    }
                    Err(_) => println!("✗ 无效的PID"),
                }
            }
            "6" => {
                pm.run_one_cycle();
            }
            "7" => {
                pm.print_status();
            }
            "8" => {
                println!("\n请选择快照类型:");
                println!("  1. 初始化快照");
                println!("  2. 运行前快照");
                println!("  3. 运行后快照");
                print!("选择 (1-3): ");
                io::stdout().flush().unwrap();
                
                let mut snapshot_choice = String::new();
                io::stdin().read_line(&mut snapshot_choice).unwrap();
                
                let snapshot = match snapshot_choice.trim() {
                    "1" => pm.get_snapshot("初始化快照"),
                    "2" => pm.get_snapshot("运行前快照"),
                    "3" => pm.get_snapshot("运行后快照"),
                    _ => {
                        println!("✗ 无效选择");
                        continue;
                    }
                };
                
                println!("{}", snapshot);
            }
            "9" => {
                println!("创建5个测试进程...");
                for i in 1..=5 {
                    let priority = i as u32;
                    match pm.create_process(priority) {
                        Ok(pid) => println!("  ✓ 进程 {} 创建成功 (优先级: {})", pid, priority),
                        Err(e) => println!("  ✗ 创建进程失败: {}", e),
                    }
                }
            }
            "s" | "S" => {
                pm.show_scheduler_stats();
            }
            "0" => {
                println!("感谢使用！再见！");
                break;
            }
            _ => {
                println!("✗ 无效的选择，请重新输入");
            }
        }
    }
}
