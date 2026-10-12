use std::{collections::HashMap, env, process};

const PAGE_SIZE: usize = 16;
const VIRTUAL_SIZE: usize = 256;
const PHYSICAL_SIZE: usize = 128;

#[derive(Clone, Copy)]
enum Mode {
    Flush,
    Asid,
}

struct Access {
    pid: String,
    virtual_address: usize,
}

struct Input {
    mode: Mode,
    capacity: usize,
    page_tables: HashMap<String, HashMap<usize, usize>>,
    accesses: Vec<Access>,
}

struct TlbEntry {
    pid: String,
    vpn: usize,
    pfn: usize,
    last_used: usize,
}

struct Tlb {
    capacity: usize,
    entries: Vec<TlbEntry>,
}

enum AccessResult {
    Hit(usize),
    Miss(usize),
    PageFault,
    OutOfRange,
}

fn parse_input(args: &[String]) -> Result<Input, String> {
    // TODO: 解析模式、容量、多个进程的页表以及访问序列
    todo!()
}

impl Tlb {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn lookup(
        &mut self,
        mode: Mode,
        pid: &str,
        vpn: usize,
        now: usize,
    ) -> Option<usize> {
        // TODO: flush 模式与 asid 模式分别如何比较条目
        // TODO: 命中时更新 last_used
        todo!()
    }

    fn insert(&mut self, pid: &str, vpn: usize, pfn: usize, now: usize) {
        // TODO: 加入带 PID 的条目，并实现 LRU 淘汰
        todo!()
    }
}

fn translate(
    tlb: &mut Tlb,
    mode: Mode,
    page_table: &HashMap<usize, usize>,
    pid: &str,
    virtual_address: usize,
    now: usize,
) -> AccessResult {
    // TODO: TLB 查询、页表查询和物理地址计算
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });

    let mut tlb = Tlb::new(input.capacity);
    let mut current_pid: Option<String> = None;
    let mut switches = 0;
    let mut flushes = 0;
    let mut hits = 0;
    let mut misses = 0;

    for (now, access) in input.accesses.into_iter().enumerate() {
        // TODO: 检测上下文切换
        // TODO: flush 模式下清空 TLB
        // TODO: 找到该进程的页表并调用 translate
        // TODO: 输出结果并更新统计
        let _ = (now, access);
    }

    // TODO: 输出统计信息
    let _ = (
        &mut tlb,
        &mut current_pid,
        &mut switches,
        &mut flushes,
        &mut hits,
        &mut misses,
    );
}