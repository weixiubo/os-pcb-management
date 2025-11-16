#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Ready,      // 就绪
    Running,    // 运行
    Waiting,    // 等待/挂起（进程被挂起后进入等待队列）
}

impl ProcessState {
    pub fn to_string(&self) -> &str {
        match self {
            ProcessState::Ready => "就绪",
            ProcessState::Running => "运行",
            ProcessState::Waiting => "等待",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PCB {
    pub pid: u32,                    // 进程ID
    pub priority: u32,               // 优先级
    pub state: ProcessState,         // 进程状态
    pub remaining_time: u32,         // 剩余时间片
    pub pool_index: usize,           // 在PCB池中的索引（用于伙伴系统）
}

impl PCB {
    pub fn new(pool_index: usize, pid: u32) -> Self {
        PCB {
            pid,
            priority: 0,
            state: ProcessState::Ready,
            remaining_time: 0,
            pool_index,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.pid = 0;
        self.priority = 0;
        self.state = ProcessState::Ready;
        self.remaining_time = 0;
    }

    pub fn display(&self) -> String {
        format!(
            "PID: {}, 优先级: {}, 状态: {}, 剩余时间片: {}",
            self.pid,
            self.priority,
            self.state.to_string(),
            self.remaining_time
        )
    }
}
