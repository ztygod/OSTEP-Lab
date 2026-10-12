## 周六：单级页表翻译器

### 1. 前置知识／相关理论

单级页表按虚拟页号（VPN）查表。有效页表项保存物理页帧号（PFN）；无效页表项意味着该虚拟页没有映射。阅读 OSTEP [`vm-paging/paging-linear-translate.py`](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/vm-paging/paging-linear-translate.py)，定位它如何拆分虚拟地址、检查页表项有效位、组合物理地址。配套的 [实验说明](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/vm-paging/README.md) 解释了页表项格式。[research.cs.wisc.edu](https://research.cs.wisc.edu/wind/OSTEP/Homework/homework.html?utm_source=chatgpt.com)

### 2. 具体要求

1. 创建 Rust 项目 `linear-page-table`，只使用标准库。命令行接收两个参数：映射列表和待翻译地址列表，例如 `0:3,1:6,7:0` 表示 VPN 0→PFN 3、VPN 1→PFN 6、VPN 7→PFN 0。
2. 建立 **16 项**单级页表。没有指定映射的项视为无效；PFN `0` 是合法页帧，不能用数值 `0` 表示“无效”。
3. 逐个翻译十进制虚拟地址。未映射时输出 `FAULT`；虚拟地址达到或超过 `256` 时输出 `OUT_OF_RANGE`。
4. 拒绝重复 VPN、VPN 超出 `0..15`、PFN 超出 `0..7`、空映射项及非数字参数。配置错误应打印到标准错误并以非零状态退出。

### 3. 验收标准

环境：Linux、macOS 或 Ubuntu 容器；稳定版 Rust，能够运行 `cargo`。在 `linear-page-table` 目录执行：

```bash
cargo run --quiet -- '0:3,1:6,7:0' '0,31,112,32,256'
```

标准输出应**逐行**为：

```text
0 -> 48
31 -> 111
112 -> 0
32 -> FAULT
256 -> OUT_OF_RANGE
```

重点检查 `31`：它位于 VPN 1，偏移是 15，因此结果为 `6×16+15=111`。再执行 `cargo run --quiet -- '0:8' '0'`，应非零退出，因为物理内存只有编号 `0..7` 的八个页帧。

**交付物：**可运行的 Rust 项目，以及一段写在 `README.md` 中的说明：VPN、偏移、PFN 分别怎样参与翻译。

### 4. 在实验目录生成测试用例的 prompt

> 请以当前 `linear-page-table` 文件夹为工作目录，先阅读 `README.md`、`src/` 和已有测试，只为本实验创建测试，不修改实现代码。程序通过 `cargo run --quiet -- '<VPN:PFN映射列表>' '<十进制虚拟地址列表>'` 运行；虚拟空间为 256 字节，物理内存为 128 字节，页大小为 16 字节。请在 `tests/` 编写可通过 `cargo test` 运行的集成测试，覆盖页首与页末偏移、PFN 为 0、已映射与未映射页、最大合法虚拟地址、越界地址，以及重复 VPN、越界 VPN/PFN、空项和非数字输入。根据规格中的地址计算公式独立推导期望结果，不要运行现有程序后把其输出抄成断言。错误输入检查非零退出和有意义的标准错误信息；不要替我完成翻译器实现。最后说明测试运行命令和每类用例验证的规则。

### 5. 最原始的代码骨架

先运行 `cargo new linear-page-table`，再将 `src/main.rs` 换为：

```rust
use std::{env, process};

const PAGE_SIZE: usize = 16;
const VIRTUAL_SIZE: usize = 256;
const PHYSICAL_SIZE: usize = 128;

struct Input {
    table: [Option<usize>; VIRTUAL_SIZE / PAGE_SIZE],
    addresses: Vec<usize>,
}

enum Translation {
    Physical(usize),
    Fault,
    OutOfRange,
}

fn parse_input(args: &[String]) -> Result<Input, String> {
    // TODO: 解析映射和地址，并检查重复或越界的页号
    todo!()
}

fn translate(table: &[Option<usize>], virtual_address: usize) -> Translation {
    // TODO: 拆分 VPN 与偏移、查页表、组合物理地址
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    for address in input.addresses {
        let result = match translate(&input.table, address) {
            Translation::Physical(pa) => pa.to_string(),
            Translation::Fault => "FAULT".to_string(),
            Translation::OutOfRange => "OUT_OF_RANGE".to_string(),
        };
        println!("{address} -> {result}");
    }
}
```