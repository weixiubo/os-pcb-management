use crate::pcb::PCB;

/// 进程队列：使用链表实现
pub struct ProcessQueue {
    head: Option<Box<Node>>,
    length: usize,
}

struct Node {
    pcb: PCB,
    next: Option<Box<Node>>,
}

impl ProcessQueue {
    pub fn new() -> Self {
        ProcessQueue {
            head: None,
            length: 0,
        }
    }

    /// 普通入队（FIFO）
    pub fn enqueue(&mut self, pcb: PCB) {
        let new_node = Box::new(Node {
            pcb,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_node);
        }

        self.length += 1;
    }

    /// 按优先级入队（优先级高的在前面）
    /// 用于就绪队列，实现优先级调度
    pub fn enqueue_by_priority(&mut self, pcb: PCB) {
        let mut new_node = Box::new(Node {
            pcb,
            next: None,
        });

        // 如果队列为空或新进程优先级高于队首
        if self.head.is_none() || new_node.pcb.priority > self.head.as_ref().unwrap().pcb.priority {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            // 找到合适的插入位置
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() &&
                  current.next.as_ref().unwrap().pcb.priority >= new_node.pcb.priority {
                current = current.next.as_mut().unwrap();
            }
            new_node.next = current.next.take();
            current.next = Some(new_node);
        }

        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Option<PCB> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.length -= 1;
            Some(node.pcb)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&PCB> {
        self.head.as_ref().map(|node| &node.pcb)
    }

    pub fn front_mut(&mut self) -> Option<&mut PCB> {
        self.head.as_mut().map(|node| &mut node.pcb)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    /// 根据PID查找并移除进程
    pub fn remove_by_pid(&mut self, pid: u32) -> Option<PCB> {
        if let Some(node) = &mut self.head {
            if node.pcb.pid == pid {
                // 移除头节点
                let mut node = self.head.take().unwrap();
                self.head = node.next.take();
                self.length -= 1;
                return Some(node.pcb);
            }
        }

        let mut current = self.head.as_mut()?;
        while let Some(next) = &mut current.next {
            if next.pcb.pid == pid {
                let mut node = current.next.take().unwrap();
                current.next = node.next.take();
                self.length -= 1;
                return Some(node.pcb);
            }
            current = current.next.as_mut().unwrap();
        }

        None
    }

    /// 根据PID查找进程（可变引用）
    /// 注意：由于Rust的借用检查限制，此函数暂不使用
    /// 改用remove_by_pid + enqueue的方式更新
    #[allow(dead_code)]
    pub fn find_mut_by_pid(&mut self, _pid: u32) -> Option<&mut PCB> {
        // 此函数由于借用检查限制暂时不使用
        // 实际使用中通过remove_by_pid + enqueue来更新
        None
    }

    /// 获取队列中最后一个进程的PID
    #[allow(dead_code)]
    pub fn get_last_pid(&self) -> Option<u32> {
        let mut current = self.head.as_ref()?;
        while let Some(next) = &current.next {
            current = next;
        }
        Some(current.pcb.pid)
    }

    /// 打印队列中的所有进程
    pub fn print_all(&self) {
        if self.is_empty() {
            println!("  (空)");
            return;
        }

        let mut current = self.head.as_ref();
        let mut index = 1;
        while let Some(node) = current {
            println!("  [{}] {}", index, node.pcb.display());
            current = node.next.as_ref();
            index += 1;
        }
    }

    /// 克隆队列（用于快照）
    pub fn clone(&self) -> Self {
        let mut new_queue = ProcessQueue::new();
        let mut current = self.head.as_ref();
        
        while let Some(node) = current {
            new_queue.enqueue(node.pcb.clone());
            current = node.next.as_ref();
        }
        
        new_queue
    }
}

impl Clone for ProcessQueue {
    fn clone(&self) -> Self {
        let mut new_queue = ProcessQueue::new();
        let mut current = self.head.as_ref();
        
        while let Some(node) = current {
            new_queue.enqueue(node.pcb.clone());
            current = node.next.as_ref();
        }
        
        new_queue
    }
}
