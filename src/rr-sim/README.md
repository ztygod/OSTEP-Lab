## 2026.9.26 实现轮转调度器

### 1. 前置知识／相关理论

轮转调度（RR）维护一个就绪队列。每次取队首任务，最多运行一个时间片；任务没完成就放回队尾，完成则退出队列。先读 OSTEP [`cpu-sched/scheduler.py`](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/cpu-sched/scheduler.py) 中 `if options.policy == 'RR'` 的分支，重点看 `runlist.pop(0)`、运行时间的计算和重新入队。:chatgpt-content-reference{index="0"}

### 2. 具体要求

1. 新建 `rr-sim` Rust 项目。命令行接收两个参数：**时间片**和以逗号分隔的**任务运行时长**。所有任务在时间 `0` 到达，编号按输入顺序从 `0` 开始。
2. 用就绪队列实现 RR。按每个时间单位输出任务编号；不计上下文切换开销。
3. 记录每个任务首次运行的时间（响应时间）和完成时间。拒绝时间片为 `0`、空任务列表及运行时长为 `0` 的输入。
4. 完成后用 OSTEP 的 `python3 scheduler.py -p RR -l 5,3,1 -q 2 -c` 对照运行片段和完成时间。OSTEP 使用任务编号，你的输出格式按下面约定即可。:chatgpt-content-reference{index="1"}

### 3. 验收标准

环境：Linux、macOS 或你现有的 Ubuntu 容器；安装可运行 `cargo` 的稳定版 Rust。下面命令均在 **`rr-sim` 目录**执行：

```bash
cargo run --quiet -- 2 5,3,1
```

标准输出必须为：

```text
trace: 0 0 1 1 2 0 0 1 0
finish: 0=9 1=8 2=5
response: 0=0 1=2 2=4
```

再运行 `cargo run --quiet -- 2 3`，应得到 `trace: 0 0 0`、`finish: 0=3`、`response: 0=0`。运行 `cargo run --quiet -- 0 5,3` 应以非零状态退出，并在标准错误中说明时间片必须大于 `0`；非法输入不能造成死循环。

### 4. 生成测试用例的完整 prompt

在 `rr-sim` 目录打开你的编程助手，直接粘贴：

> 请以当前目录作为 Rust 实验项目根目录，先阅读 `src/main.rs` 和项目已有的测试，再为这个轮转调度器编写测试。不要实现或改写调度器，也不要修改 `src/`。程序通过 `cargo run --quiet -- <时间片> <逗号分隔的任务时长>` 运行；所有任务在时间 0 到达，按输入顺序编号，从就绪队列队首取任务，最多运行一个时间片，未完成则放回队尾。标准输出有 `trace`、`finish`、`response` 三行。请在 `tests/` 下创建可通过 `cargo test` 执行的集成测试，覆盖多任务轮转、任务恰好在时间片末完成、单任务、时间片大于全部任务时长，以及时间片为 0、空列表、时长为 0 和非数字输入。根据明确的调度规则独立推导预期值，不要读取程序当前输出后原样写成断言。错误输入应检查退出状态和标准错误，不要依赖错误文案的完整逐字匹配。最后告诉我运行测试的命令，以及每组测试在验证什么；不要替我完成实验实现。

### 5. 最原始的代码骨架

运行 `cargo new rr-sim`，将 `src/main.rs` 改为以下骨架。`TODO` 留给你完成：

```rust
use std::{env, process};

struct Report {
    trace: Vec<usize>,
    finish: Vec<u32>,
    response: Vec<u32>,
}

fn parse_input(args: &[String]) -> Result<(u32, Vec<u32>), String> {
    // TODO: 解析并校验时间片与任务时长
    todo!()
}

fn simulate(quantum: u32, lengths: &[u32]) -> Report {
    // TODO: 建立就绪队列，执行轮转调度并记录时间
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (quantum, lengths) = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    let result = simulate(quantum, &lengths);
    let trace = result.trace.iter().map(usize::to_string).collect::<Vec<_>>();
    let finish = result.finish.iter().enumerate()
        .map(|(id, time)| format!("{id}={time}")).collect::<Vec<_>>();
    let response = result.response.iter().enumerate()
        .map(|(id, time)| format!("{id}={time}")).collect::<Vec<_>>();
    println!("trace: {}", trace.join(" "));
    println!("finish: {}", finish.join(" "));
    println!("response: {}", response.join(" "));
}
```