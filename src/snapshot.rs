use crate::queue::ProcessQueue;
use crate::pcb::PCB;

/// 系统快照：记录系统在某一时刻的状态
///
/// 快照类型：
/// 1. 初始化快照：系统刚启动，PCB池满，运行队列空
/// 2. 运行前快照：进程执行前的状态
/// 3. 运行后快照：进程执行后的状态
pub struct Snapshot {
    name: String,
    free_pcb_count: usize,
    used_pcb_count: usize,
    total_chain: Vec<PCB>,          // 总链中的所有进程
    ready_queue: ProcessQueue,
    waiting_queue: ProcessQueue,
    running_queue: ProcessQueue,
}

impl Snapshot {
    pub fn new(
        name: &str,
        free_pcb_count: usize,
        used_pcb_count: usize,
        total_chain: Vec<PCB>,
        ready_queue: ProcessQueue,
        waiting_queue: ProcessQueue,
        running_queue: ProcessQueue,
    ) -> Self {
        Snapshot {
            name: name.to_string(),
            free_pcb_count,
            used_pcb_count,
            total_chain,
            ready_queue,
            waiting_queue,
            running_queue,
        }
    }

    pub fn display(&self) -> String {
        let mut result = format!(
            "\n╔═══════════════════════════════════════════════════════╗\n\
             ║ 📸 {:<50}║\n\
             ╠═══════════════════════════════════════════════════════╣\n\
             ║ PCB池状态: 空闲 {:>3} / 已用 {:>3} / 总计 {:>3}          ║\n\
             ║ 总链: {:>3} 个进程                                      ║\n\
             ║ 就绪队列: {:>3} 个进程                                  ║\n\
             ║ 等待队列: {:>3} 个进程                                  ║\n\
             ║ 运行队列: {:>3} 个进程                                  ║\n\
             ╚═══════════════════════════════════════════════════════╝",
            self.name,
            self.free_pcb_count,
            self.used_pcb_count,
            self.free_pcb_count + self.used_pcb_count,
            self.total_chain.len(),
            self.ready_queue.len(),
            self.waiting_queue.len(),
            self.running_queue.len(),
        );

        // 如果有进程，显示详细信息
        if !self.total_chain.is_empty() {
            result.push_str("\n\n总链中的进程详情：");
            for (i, pcb) in self.total_chain.iter().enumerate() {
                result.push_str(&format!("\n  [{}] {}", i + 1, pcb.display()));
            }
        }

        result
    }
}

impl std::fmt::Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
