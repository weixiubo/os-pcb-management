use crate::pcb::PCB;

/// 伙伴系统：用于管理PCB池
/// 使用伙伴系统算法来分配和回收PCB块
///
/// 算法原理：
/// 1. 将内存分成2^n大小的块
/// 2. 维护多级空闲链表，每级对应不同大小的块
/// 3. 分配时从合适大小的空闲链表中取块，必要时分裂大块
/// 4. 回收时尝试与伙伴块合并，形成更大的空闲块
pub struct BuddySystem {
    pool: Vec<Option<PCB>>,         // PCB池，实际存储PCB对象
    free_list: Vec<Vec<usize>>,     // 按大小分组的空闲块列表，free_list[k]存储大小为2^k的空闲块起始索引
    max_order: usize,               // 最大阶数（2^max_order = pool_size）
    pool_size: usize,               // 池的实际大小（2的幂）
    used_count: usize,              // 已使用的PCB数量
}

impl BuddySystem {
    pub fn new(requested_size: usize) -> Self {
        // 计算大于等于requested_size的最小2的幂
        let mut pool_size = 1;
        let mut max_order = 0;

        while pool_size < requested_size {
            pool_size <<= 1;
            max_order += 1;
        }

        // 初始化空闲列表，从0阶到max_order阶
        let mut free_list = vec![Vec::new(); max_order + 1];

        // 初始时，整个池是一个大的空闲块，放在最高阶
        free_list[max_order].push(0);

        println!("伙伴系统初始化: 池大小={}, 最大阶数={}", pool_size, max_order);

        BuddySystem {
            pool: vec![None; pool_size],
            free_list,
            max_order,
            pool_size,
            used_count: 0,
        }
    }

    /// 分配一个PCB块（扩展一：伙伴系统分配算法）
    /// 返回分配的索引，如果分配失败返回None
    pub fn allocate(&mut self) -> Option<usize> {
        // 我们总是分配大小为1的块（0阶）
        let order = 0;

        // 查找最小的可用块
        let mut alloc_order = order;
        while alloc_order <= self.max_order {
            if !self.free_list[alloc_order].is_empty() {
                break;
            }
            alloc_order += 1;
        }

        // 如果没有可用块
        if alloc_order > self.max_order {
            return None;
        }

        // 从找到的阶数中取出一个块
        let index = self.free_list[alloc_order].pop().unwrap();

        // 如果找到的块比需要的大，需要分裂
        while alloc_order > order {
            alloc_order -= 1;
            let buddy_index = index + (1 << alloc_order);
            // 将分裂出的伙伴块加入对应阶的空闲列表
            self.free_list[alloc_order].push(buddy_index);
        }

        self.used_count += 1;
        Some(index)
    }

    /// 在指定索引存储PCB
    pub fn store_pcb(&mut self, index: usize, pcb: PCB) {
        if index < self.pool_size {
            self.pool[index] = Some(pcb);
        }
    }

    /// 从指定索引获取PCB的引用
    #[allow(dead_code)]
    pub fn get_pcb(&self, index: usize) -> Option<&PCB> {
        if index < self.pool_size {
            self.pool[index].as_ref()
        } else {
            None
        }
    }

    /// 回收PCB到池中（扩展三：PCB回收算法）
    pub fn deallocate(&mut self, index: usize) {
        if index >= self.pool_size {
            return;
        }

        // 清除存储的PCB
        self.pool[index] = None;
        self.used_count -= 1;

        // 尝试合并伙伴块（扩展三：空白块合并）
        self.merge_and_free(index, 0);
    }

    /// 合并伙伴块并释放（扩展三：空白块合并算法）
    ///
    /// 算法步骤：
    /// 1. 计算当前块的伙伴块索引
    /// 2. 检查伙伴块是否空闲且在同一阶
    /// 3. 如果可以合并，从空闲列表移除伙伴块，合并后递归向上合并
    /// 4. 如果不能合并，将当前块加入空闲列表
    fn merge_and_free(&mut self, mut index: usize, mut order: usize) {
        loop {
            // 如果已经是最大阶，直接加入空闲列表
            if order >= self.max_order {
                if !self.free_list[order].contains(&index) {
                    self.free_list[order].push(index);
                }
                break;
            }

            // 计算伙伴块的索引
            let buddy_index = index ^ (1 << order);

            // 检查伙伴块是否在同一阶的空闲列表中
            if let Some(pos) = self.free_list[order].iter().position(|&x| x == buddy_index) {
                // 伙伴块空闲，可以合并
                // 从空闲列表中移除伙伴块
                self.free_list[order].remove(pos);

                // 合并：取两个块中索引较小的作为合并后的块索引
                index = index.min(buddy_index);
                order += 1;

                // 继续尝试向上合并
            } else {
                // 伙伴块不空闲或不在同一阶，无法合并
                // 将当前块加入当前阶的空闲列表
                if !self.free_list[order].contains(&index) {
                    self.free_list[order].push(index);
                }
                break;
            }
        }
    }

    pub fn get_free_count(&self) -> usize {
        self.pool_size - self.used_count
    }

    pub fn get_used_count(&self) -> usize {
        self.used_count
    }

    pub fn get_pool_size(&self) -> usize {
        self.pool_size
    }

    /// 打印伙伴系统状态（用于调试）
    #[allow(dead_code)]
    pub fn print_status(&self) {
        println!("\n=== 伙伴系统状态 ===");
        println!("池大小: {}, 已用: {}, 空闲: {}",
                 self.pool_size, self.used_count, self.get_free_count());
        for (order, list) in self.free_list.iter().enumerate() {
            if !list.is_empty() {
                println!("  阶数 {} (大小 {}): {:?}", order, 1 << order, list);
            }
        }
    }
}
