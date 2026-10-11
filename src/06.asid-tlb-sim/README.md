## 周日实验：实现支持 ASID 的多进程 TLB

### 1. 前置知识／相关理论

不同进程可以拥有相同的虚拟地址，但这些地址可能映射到不同的物理页帧。因此，只有 VPN 的 TLB 条目不能直接在进程间共享。

传统处理方法是在切换地址空间时清空 TLB。这样可以保证正确性，但新进程需要重新产生一批 TLB 未命中。

另一种方法是在 TLB 条目中加入 ASID：

```text
(ASID, VPN) -> PFN
```

查找时同时比较 ASID 和 VPN，不同进程便可以安全地保留各自的翻译缓存。OSTEP 的 TLB 教学材料将“切换时清空 TLB”和“为每个条目加入 ASID”列为处理上下文切换的两种办法。[pages.cs.wisc.edu](https://pages.cs.wisc.edu/~remzi/OSTEP/Educators-Slides/Andrea/lecture6-TLB.pdf?utm_source=chatgpt.com)

RISC-V 的 `satp` 寄存器包含页表根地址和 ASID。规范还规定，当一个 ASID 被重新分配给另一张页表时，软件需要执行相应的 `SFENCE.VMA`，避免继续使用旧映射。[GitHub](https://github.com/riscv/riscv-isa-manual/blob/main/src/priv/supervisor.adoc?utm_source=chatgpt.com)

源码观察点：

- [RISC-V 特权级规范中 `satp` 与 `SFENCE.VMA`](https://github.com/riscv/riscv-isa-manual/blob/main/src/priv/supervisor.adoc)
- [xv6 `kernel/vm.c`](https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/vm.c)
- [xv6 页表章节](https://mit-pdos.github.io/xv6-riscv-book/mem.html)

### 2. 具体要求

1. 创建 Rust 项目 `asid-tlb-sim`。

2. 支持两个运行模式：

   - `flush`：进程发生变化时清空整个 TLB；
   - `asid`：TLB 条目使用 `(进程ID, VPN)` 作为键，进程切换不清空。

3. 输入包含：

   - 模式；
   - TLB 容量；
   - 多个进程的页表；
   - 带进程ID的地址访问序列。

4. 命令格式：

```bash
cargo run --quiet -- <mode> <capacity> \
  '<pid=vpn:pfn,...;pid=vpn:pfn,...>' \
  '<pid:va,...>'
```

5. 进程标识使用非空字符串，例如 `A`、`B`。本实验可直接把进程ID当作 ASID。

6. 每次访问时：

   1. 判断当前进程是否改变；
   2. `flush` 模式下，进程变化时清空 TLB；
   3. 使用当前进程的页表；
   4. `asid` 模式按 `(PID, VPN)` 查找；
   5. 继续使用周六实验的 LRU 策略。

7. 输出每次访问的进程、VA、命中情况和物理地址。最后输出：

   - 上下文切换次数；
   - TLB 清空次数；
   - 命中与未命中次数。

8. 必须保证进程 A 的 `(VPN 0 → PFN 3)` 不会被进程 B 的 VPN 0 错误命中。

9. 拒绝未知模式、重复进程、重复映射、引用不存在的进程及非法地址。

### 3. 验收标准

运行无 ASID 的清空模式：

```bash
cargo run --quiet -- flush 4 \
  'A=0:3,1:4;B=0:6,1:2' \
  'A:0,B:0,A:16,B:16,A:0,B:0'
```

预期输出：

```text
pid=A va=0 MISS pa=48
pid=B va=0 MISS pa=96
pid=A va=16 MISS pa=64
pid=B va=16 MISS pa=32
pid=A va=0 MISS pa=48
pid=B va=0 MISS pa=96
stats: switches=5 flushes=5 hits=0 misses=6
```

因为访问序列每次都切换进程，所以每次访问前，前一个进程留下的条目都会被清除。

运行 ASID 模式：

```bash
cargo run --quiet -- asid 4 \
  'A=0:3,1:4;B=0:6,1:2' \
  'A:0,B:0,A:16,B:16,A:0,B:0'
```

预期输出：

```text
pid=A va=0 MISS pa=48
pid=B va=0 MISS pa=96
pid=A va=16 MISS pa=64
pid=B va=16 MISS pa=32
pid=A va=0 HIT pa=48
pid=B va=0 HIT pa=96
stats: switches=5 flushes=0 hits=2 misses=4
```

还要验证地址空间隔离：

```bash
cargo run --quiet -- asid 2 'A=0:3;B=0:6' 'A:0,B:0,A:0'
```

预期输出：

```text
pid=A va=0 MISS pa=48
pid=B va=0 MISS pa=96
pid=A va=0 HIT pa=48
stats: switches=2 flushes=0 hits=1 misses=2
```

如果最后一次 A 得到 `96`，说明实现只按 VPN 查找，错误地复用了 B 的映射。

交付物：

- 完整 Rust 项目；
- `flush` 与 `asid` 两种模式的输出；
- README 中回答两个问题：

  1. 为什么进程切换后直接保留不带 ASID 的 TLB 会产生错误？
  2. 为什么 ASID 被重新分配时仍可能需要刷新对应的 TLB 条目？

### 4. 生成测试用例的完整 Prompt

> 请以当前 `asid-tlb-sim` 文件夹为工作目录，先阅读`README.md`、`Cargo.toml`、`src/`和现有测试。只生成测试，不修改或补全实现代码。
>
> 该程序模拟多进程TLB，命令格式为`cargo run --quiet -- <flush|asid> <容量> '<进程页表>' '<访问序列>'`。`flush`模式在当前进程变化时清空TLB；`asid`模式使用`(PID, VPN)`标识TLB条目，切换时保留条目。TLB全相联并使用LRU淘汰。虚拟地址空间为256字节，物理内存为128字节，页大小为16字节。
>
> 请使用Rust标准库在`tests/`下创建能通过`cargo test`运行的集成测试。覆盖：单进程访问、频繁上下文切换、flush模式重新产生未命中、asid模式跨切换命中、两个进程使用相同VPN但映射到不同PFN、ASID条目参与LRU淘汰、页故障、地址越界、未知模式、容量为0、重复进程、重复VPN、引用不存在的进程以及非数字参数。
>
> 请独立根据地址翻译、进程切换、ASID和LRU规则推导期望结果，不能运行当前程序后复制其输出。测试必须能够发现“只比较VPN而忽略PID”的错误实现。错误输入检查非零退出及有意义的错误信息，不要求逐字匹配。不要替我实现`src/`中的TODO。最后说明测试命令和每组测试要捕获的错误。

### 5. 最原始简单的代码骨架

```bash
cargo new asid-tlb-sim
cd asid-tlb-sim
```

将 `src/main.rs` 替换为：

```rust
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
```

本周完成后，你应该能够把下面这条链路完整说清楚：

> CPU 发出虚拟地址 → 先查 TLB → 未命中才查页表 → 把结果缓存到 TLB → 进程切换时通过清空或 ASID 保证地址空间隔离。