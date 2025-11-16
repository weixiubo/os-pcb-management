mod pcb;
mod buddy_system;
mod queue;
mod scheduler;
mod snapshot;
mod menu;

use pcb::{ProcessState, PCB};
use buddy_system::BuddySystem;
use queue::ProcessQueue;
use scheduler::Scheduler;
use snapshot::Snapshot;
use std::collections::HashMap;

const MAX_PCB_COUNT: usize = 128; // PCB池的最大容量

struct ProcessManager {
    pcb_pool: BuddySystem,
    total_chain: HashMap<u32, PCB>,  // PCB总链：用HashMap维护所有进程，key是PID
    ready_queue: ProcessQueue,       // 就绪队列
    waiting_queue: ProcessQueue,     // 等待队列
    running_queue: ProcessQueue,     // 运行队列（通常只有一个）
    scheduler: Scheduler,
    next_pid: u32,
}

impl ProcessManager {
    fn new() -> Self {
        let pm = ProcessManager {
            pcb_pool: BuddySystem::new(MAX_PCB_COUNT),
            total_chain: HashMap::new(),
            ready_queue: ProcessQueue::new(),
            waiting_queue: ProcessQueue::new(),
            running_queue: ProcessQueue::new(),
            scheduler: Scheduler::new(),
            next_pid: 1,
        };

        pm
    }

    /// 创建进程原语
    ///
    /// 步骤：
    /// 1. 从伙伴系统分配一个PCB块
    /// 2. 创建新的PCB并存储到池中
    /// 3. 加入总链（HashMap）
    /// 4. 加入就绪队列
    fn create_process(&mut self, priority: u32) -> Result<u32, String> {
        // 从伙伴系统分配一个PCB块
        let pool_index = self.pcb_pool.allocate()
            .ok_or("PCB池已满，无法创建新进程")?;

        let pid = self.next_pid;
        self.next_pid += 1;

        let mut new_pcb = PCB::new(pool_index, pid);
        new_pcb.priority = priority;
        new_pcb.state = ProcessState::Ready;
        new_pcb.remaining_time = 5; // 默认时间片

        // 存储到PCB池中
        self.pcb_pool.store_pcb(pool_index, new_pcb.clone());

        // 加入总链（HashMap）
        self.total_chain.insert(pid, new_pcb.clone());

        // 加入就绪队列（按优先级插入）
        self.ready_queue.enqueue_by_priority(new_pcb);

        println!("✓ 进程 {} 创建成功，优先级: {}, PCB索引: {}", pid, priority, pool_index);
        Ok(pid)
    }

    /// 撤销进程原语
    ///
    /// 步骤：
    /// 1. 从总链中查找进程
    /// 2. 从所有队列中移除
    /// 3. 回收PCB到伙伴系统
    fn terminate_process(&mut self, pid: u32) -> Result<(), String> {
        // 从总链中查找并移除
        let pcb = self.total_chain.remove(&pid)
            .ok_or(format!("进程 {} 不存在", pid))?;

        // 从各个队列中移除
        self.ready_queue.remove_by_pid(pid);
        self.waiting_queue.remove_by_pid(pid);
        self.running_queue.remove_by_pid(pid);

        // 回收PCB到伙伴系统
        let pool_index = pcb.pool_index;
        self.pcb_pool.deallocate(pool_index);

        println!("✓ 进程 {} 已撤销，PCB索引 {} 已回收", pid, pool_index);
        Ok(())
    }

    /// 时间片到原语
    ///
    /// 步骤：
    /// 1. 从运行队列取出进程
    /// 2. 重置时间片，状态改为就绪
    /// 3. 更新总链中的状态
    /// 4. 加入就绪队列
    /// 5. 记录进程切换
    fn time_slice_expired(&mut self) -> Result<(), String> {
        let mut running_pcb = self.running_queue.dequeue()
            .ok_or("没有正在运行的进程")?;

        running_pcb.state = ProcessState::Ready;
        running_pcb.remaining_time = 5; // 重置时间片

        // 更新总链中的状态
        if let Some(pcb_in_chain) = self.total_chain.get_mut(&running_pcb.pid) {
            pcb_in_chain.state = ProcessState::Ready;
            pcb_in_chain.remaining_time = 5;
        }

        // 按优先级加入就绪队列
        self.ready_queue.enqueue_by_priority(running_pcb.clone());

        // 记录进程切换
        self.scheduler.record_switch();

        println!("✓ 进程 {} 时间片到，转为就绪状态", running_pcb.pid);
        Ok(())
    }

    /// 挂起进程原语
    ///
    /// 步骤：
    /// 1. 从总链中查找进程
    /// 2. 从就绪或运行队列中移除
    /// 3. 状态改为等待
    /// 4. 加入等待队列
    fn suspend_process(&mut self, pid: u32) -> Result<(), String> {
        // 从总链中查找
        let pcb = self.total_chain.get_mut(&pid)
            .ok_or(format!("进程 {} 不存在", pid))?;

        // 从就绪或运行队列中移除
        let removed_from_ready = self.ready_queue.remove_by_pid(pid).is_some();
        let removed_from_running = self.running_queue.remove_by_pid(pid).is_some();

        if !removed_from_ready && !removed_from_running {
            return Err(format!("进程 {} 不在就绪或运行队列中", pid));
        }

        pcb.state = ProcessState::Waiting;

        // 加入等待队列
        self.waiting_queue.enqueue(pcb.clone());

        println!("✓ 进程 {} 已挂起", pid);
        Ok(())
    }

    /// 激活进程原语
    ///
    /// 步骤：
    /// 1. 从等待队列中移除
    /// 2. 状态改为就绪
    /// 3. 更新总链中的状态
    /// 4. 加入就绪队列
    fn activate_process(&mut self, pid: u32) -> Result<(), String> {
        let mut pcb = self.waiting_queue.remove_by_pid(pid)
            .ok_or(format!("进程 {} 不在等待队列中", pid))?;

        pcb.state = ProcessState::Ready;

        // 更新总链中的状态
        if let Some(pcb_in_chain) = self.total_chain.get_mut(&pid) {
            pcb_in_chain.state = ProcessState::Ready;
        }

        // 按优先级加入就绪队列
        self.ready_queue.enqueue_by_priority(pcb);
        println!("✓ 进程 {} 已激活，转为就绪状态", pid);
        Ok(())
    }

    /// 调度进程（扩展二：进程调度程序）
    ///
    /// 使用优先级调度策略：
    /// 1. 从就绪队列选择优先级最高的进程（队列已按优先级排序）
    /// 2. 将其状态改为运行
    /// 3. 加入运行队列
    /// 4. 记录进程切换
    fn schedule(&mut self) -> Result<(), String> {
        // 如果运行队列为空，从就绪队列中选择一个进程运行
        if self.running_queue.is_empty() {
            let mut pcb = self.ready_queue.dequeue()
                .ok_or("就绪队列为空，无法调度")?;

            pcb.state = ProcessState::Running;

            // 更新总链中的状态
            if let Some(pcb_in_chain) = self.total_chain.get_mut(&pcb.pid) {
                pcb_in_chain.state = ProcessState::Running;
            }

            self.running_queue.enqueue(pcb.clone());

            // 记录进程切换（从就绪到运行）
            self.scheduler.record_switch();

            // 使用调度器执行
            self.scheduler.execute_process(&pcb);
            Ok(())
        } else {
            // 如果已有运行进程，继续执行
            if let Some(pcb) = self.running_queue.front() {
                self.scheduler.execute_process(pcb);
                Ok(())
            } else {
                Err("运行队列异常".to_string())
            }
        }
    }

    /// 执行一次调度周期
    ///
    /// 步骤：
    /// 1. 如果没有运行进程，从就绪队列调度一个
    /// 2. 执行当前运行进程
    /// 3. 减少时间片
    /// 4. 如果时间片用完，执行时间片到原语
    fn run_one_cycle(&mut self) {
        println!("\n=== 执行一次调度周期 ===");

        // 如果运行队列为空，尝试调度
        if self.running_queue.is_empty() {
            if let Err(e) = self.schedule() {
                println!("{}", e);
                return;
            }
        }

        // 执行当前运行进程
        if let Some(pcb) = self.running_queue.front() {
            println!("正在执行进程 {} (优先级: {}, 剩余时间片: {})",
                     pcb.pid, pcb.priority, pcb.remaining_time);

            let pid = pcb.pid;

            // 减少剩余时间片
            if let Some(pcb_mut) = self.running_queue.front_mut() {
                pcb_mut.remaining_time -= 1;

                // 同步更新总链中的时间片
                if let Some(pcb_in_chain) = self.total_chain.get_mut(&pid) {
                    pcb_in_chain.remaining_time = pcb_mut.remaining_time;
                }
            }

            // 如果时间片用完，执行时间片到原语
            if let Some(pcb) = self.running_queue.front() {
                if pcb.remaining_time <= 0 {
                    if let Err(e) = self.time_slice_expired() {
                        println!("{}", e);
                    }
                }
            }
        }
    }

    /// 获取快照
    fn get_snapshot(&self, name: &str) -> Snapshot {
        // 将HashMap转换为Vec用于快照
        let total_chain_vec: Vec<PCB> = self.total_chain.values().cloned().collect();

        Snapshot::new(
            name,
            self.pcb_pool.get_free_count(),
            self.pcb_pool.get_used_count(),
            total_chain_vec,
            self.ready_queue.clone(),
            self.waiting_queue.clone(),
            self.running_queue.clone(),
        )
    }

    /// 打印当前状态
    fn print_status(&self) {
        println!("\n========== 系统状态 ==========");
        println!("PCB池: 空闲 {} / 已用 {} / 总计 {}",
                 self.pcb_pool.get_free_count(),
                 self.pcb_pool.get_used_count(),
                 self.pcb_pool.get_pool_size());
        println!("总链: {} 个进程", self.total_chain.len());
        println!("就绪队列: {} 个进程", self.ready_queue.len());
        println!("等待队列: {} 个进程", self.waiting_queue.len());
        println!("运行队列: {} 个进程", self.running_queue.len());

        if !self.running_queue.is_empty() {
            if let Some(pcb) = self.running_queue.front() {
                println!("当前运行进程: PID={}, 优先级={}, 剩余时间片={}",
                         pcb.pid, pcb.priority, pcb.remaining_time);
            }
        }

        println!("\n--- 总链详情（所有进程） ---");
        if self.total_chain.is_empty() {
            println!("  (空)");
        } else {
            let mut processes: Vec<_> = self.total_chain.values().collect();
            processes.sort_by_key(|p| p.pid);
            for (i, pcb) in processes.iter().enumerate() {
                println!("  [{}] {}", i + 1, pcb.display());
            }
        }

        println!("\n--- 就绪队列详情（按优先级排序） ---");
        self.ready_queue.print_all();
        println!("\n--- 等待队列详情 ---");
        self.waiting_queue.print_all();
        println!("\n--- 运行队列详情 ---");
        self.running_queue.print_all();
        println!("==============================\n");
    }

    /// 显示调度器统计信息
    fn show_scheduler_stats(&self) {
        let stats = self.scheduler.get_stats();
        stats.display();
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║    进程PCB组织、管理与调度模拟系统                    ║");
    println!("║    操作系统实验2 - 完整实现版                        ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    let mut pm = ProcessManager::new();
    
    // 初始化快照
    let init_snapshot = pm.get_snapshot("初始化快照");
    println!("{}", init_snapshot);
    
    // 启动菜单系统
    menu::run_menu(&mut pm);
}