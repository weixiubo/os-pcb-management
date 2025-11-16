use crate::pcb::PCB;

/// 进程调度器：负责CPU的调度与控制（扩展二）
///
/// 调度策略：
/// 1. 优先级调度：从就绪队列选择优先级最高的进程
/// 2. 时间片轮转：每个进程分配固定时间片
/// 3. 抢占式调度：时间片用完后切换进程
pub struct Scheduler {
    total_executed: u32,      // 总执行次数
    total_switches: u32,      // 总切换次数
    current_time: u32,        // 当前系统时间
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            total_executed: 0,
            total_switches: 0,
            current_time: 0,
        }
    }

    /// 执行进程（扩展二：CPU调度）
    ///
    /// 模拟CPU执行一个时间单位
    pub fn execute_process(&mut self, pcb: &PCB) {
        self.total_executed += 1;
        self.current_time += 1;

        println!(
            "  [CPU调度] 执行进程 PID={}, 优先级={}, 剩余时间片={}, 系统时间={}",
            pcb.pid, pcb.priority, pcb.remaining_time, self.current_time
        );
    }

    /// 记录进程切换
    pub fn record_switch(&mut self) {
        self.total_switches += 1;
    }

    /// 获取调度统计信息
    pub fn get_stats(&self) -> SchedulerStats {
        SchedulerStats {
            total_executed: self.total_executed,
            total_switches: self.total_switches,
            current_time: self.current_time,
        }
    }

    /// 重置调度器
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.total_executed = 0;
        self.total_switches = 0;
        self.current_time = 0;
    }
}

/// 调度器统计信息
pub struct SchedulerStats {
    pub total_executed: u32,
    pub total_switches: u32,
    pub current_time: u32,
}

impl SchedulerStats {
    pub fn display(&self) {
        println!("\n=== 调度器统计信息 ===");
        println!("总执行次数: {}", self.total_executed);
        println!("总切换次数: {}", self.total_switches);
        println!("系统时间: {}", self.current_time);
        if self.total_switches > 0 {
            println!("平均执行时间: {:.2}",
                     self.total_executed as f64 / self.total_switches as f64);
        }
        println!("====================\n");
    }
}
