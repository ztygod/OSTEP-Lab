## 周日：二级页表与两种缺页位置

### 1. 前置知识／相关理论

周六的单级页表始终有 16 个表项，即使多数虚拟页未映射。周日把 4 位 VPN 拆成两段：**高 2 位选页目录项，低 2 位选二级页表项**。某个目录项没有二级表，与目录项存在但目标页表项无效，是两处不同的查找失败。

阅读 OSTEP [`vm-smalltables/paging-multilevel-translate.py`](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/vm-smalltables/paging-multilevel-translate.py) 中的 `getPageDirEntry()`、`getPageTableEntry()` 和 `translate()`：观察它先查 PDE、再查 PTE，并分别报告两种无效情况。[GitHub](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/vm-smalltables/paging-multilevel-translate.py?utm_source=chatgpt.com)

### 2. 具体要求

1. 创建 `two-level-page-table` Rust 项目，沿用周六的内存大小、页大小、命令行输入格式和地址计算规则。即使周六尚未完成，也可以从下面的骨架独立开始。
2. 用含 **4 项**的页目录表示 VPN 高 2 位。只在某个目录下首次出现映射时，为它创建含 **4 项**的二级页表。
3. 翻译时依次检查虚拟地址范围、PDE、PTE。分别输出 `OUT_OF_RANGE`、`PDE_FAULT`、`PTE_FAULT`，成功时输出物理地址，并标明访问的目录与页表索引。
4. 在所有地址结果后输出已占用的表项数：`4 + 已创建二级表数量 × 4`。这里**只统计页表元数据表项**，暂不模拟页表本身占用哪个物理页帧。

### 3. 验收标准

执行环境同周六。在 `two-level-page-table` 目录运行：

```bash
cargo run --quiet -- '0:3,1:6,7:0' '0,31,112,32,128,256'
```

标准输出应为：

```text
0 -> 48 (pde=0,pte=0)
31 -> 111 (pde=0,pte=1)
112 -> 0 (pde=1,pte=3)
32 -> PTE_FAULT
128 -> PDE_FAULT
256 -> OUT_OF_RANGE
entries: 12
```

VPN 2 对应目录 0、表项 2：二级表已存在，但该表项未映射，所以是 `PTE_FAULT`。VPN 8 对应目录 2：整个二级表都不存在，所以是 `PDE_FAULT`。本例只创建目录 0 和目录 1 对应的两张二级表，占用 `4+2×4=12` 个元数据表项；周六的单级表则固定为 16 项。

**交付物：**可运行项目，以及 `README.md` 中一张你自己画的 VPN 0、1、7 对应的“目录索引→表索引→PFN”小表。

### 4. 在实验目录生成测试用例的 prompt

> 请以当前 `two-level-page-table` 文件夹为工作目录，先阅读 `README.md`、`src/` 和现有测试，只生成测试代码，不修改实现。虚拟空间为 256 字节、页大小为 16 字节，VPN 共 4 位，高 2 位选 4 项页目录，低 2 位选每张二级表中的 4 个表项。二级表按需创建；翻译须区分 `PDE_FAULT`、`PTE_FAULT` 和 `OUT_OF_RANGE`，最后报告目录及已创建二级表的表项总数。请在 `tests/` 建立可用 `cargo test` 运行的集成测试，覆盖同一目录内多个映射、跨目录映射、PFN 为 0、两种缺页位置、边界虚拟地址、表项统计，以及重复 VPN、越界 VPN/PFN、空项与非数字输入。独立按位拆分规则和地址公式推导期望值，不能把当前程序输出直接保存为答案；不要替我实现页表。最后列出测试命令及每组用例所验证的规则。

### 5. 最原始的代码骨架

运行 `cargo new two-level-page-table`，将 `src/main.rs` 换为：

```rust
use std::{env, process};

const PAGE_SIZE: usize = 16;
const VIRTUAL_SIZE: usize = 256;
const PHYSICAL_SIZE: usize = 128;

struct TwoLevelTable {
    directory: [Option<[Option<usize>; 4]>; 4],
}

enum Translation {
    Physical { address: usize, pde: usize, pte: usize },
    PdeFault,
    PteFault,
    OutOfRange,
}

fn parse_input(args: &[String]) -> Result<(Vec<(usize, usize)>, Vec<usize>), String> {
    // TODO: 解析 VPN:PFN 映射和待翻译的地址
    todo!()
}

impl TwoLevelTable {
    fn new() -> Self {
        Self { directory: [None; 4] }
    }

    fn map(&mut self, vpn: usize, pfn: usize) -> Result<(), String> {
        // TODO: 拆分 VPN；按需创建二级表；检查重复与越界映射
        todo!()
    }

    fn translate(&self, virtual_address: usize) -> Translation {
        // TODO: 依次检查地址范围、PDE、PTE，最后组合物理地址
        todo!()
    }

    fn entry_count(&self) -> usize {
        // TODO: 目录项数 + 已创建二级表的表项数
        todo!()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mappings, addresses) = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    let mut table = TwoLevelTable::new();
    for (vpn, pfn) in mappings {
        table.map(vpn, pfn).unwrap_or_else(|error| {
            eprintln!("error: {error}");
            process::exit(1);
        });
    }
    for va in addresses {
        match table.translate(va) {
            Translation::Physical { address, pde, pte } =>
                println!("{va} -> {address} (pde={pde},pte={pte})"),
            Translation::PdeFault => println!("{va} -> PDE_FAULT"),
            Translation::PteFault => println!("{va} -> PTE_FAULT"),
            Translation::OutOfRange => println!("{va} -> OUT_OF_RANGE"),
        }
    }
    println!("entries: {}", table.entry_count());
}
```