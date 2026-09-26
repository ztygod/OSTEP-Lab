## 2026.9.27：给调度器加入 I/O 阻塞与唤醒

### 1. 前置知识／相关理论

任务等待 I/O 时处于 `BLOCKED`，不能待在就绪队列里；等待结束后才恢复为 `READY`。读 OSTEP [`cpu-intro/process-run.py`](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/cpu-intro/process-run.py) 的 `move_to_wait()`、`move_to_ready()` 和 `run()` 中检查 `io_finish_times` 的部分。它是**事件顺序的观察材料**：OSTEP 把发起 I/O 当作一条执行指令，而本实验使用下面定义的简化模型，因此不要求两者逐行输出相同。:chatgpt-content-reference{index="2"}

### 2. 具体要求

1. 新建 `io-sim` 项目。命令行第一个参数是时间片，第二个参数描述任务：分号分隔任务、逗号分隔同一任务的阶段。例如 `2,2,1;1` 表示任务 `0` 执行 **CPU 2 → I/O 2 → CPU 1**，任务 `1` 执行 **CPU 1**。每个任务必须以 CPU 阶段开始和结束，各阶段时长均大于 `0`。
2. 沿用周六的 RR 就绪队列。一个 CPU 阶段结束后，若还有 I/O 阶段，任务立即转为 `BLOCKED`；I/O 不占 CPU，从该时刻起等待指定时长，到期后加入就绪队列，接着执行下一个 CPU 阶段。
3. 没有可运行任务但仍有被阻塞任务时，时间继续推进，输出 `IDLE`。同一时刻若恰好发生 **I/O 完成**和**当前任务时间片结束**，先把唤醒任务加入队列，再把用完时间片的任务放回队尾。
4. 输出逐时间单位的 CPU 轨迹及各任务完成时间。拒绝时间片为 `0`、阶段时长为 `0`、阶段序列不以 CPU 结束等输入。可以复制周六项目中适用的数据结构与解析代码，但调度状态和事件处理要自己实现。

### 3. 验收标准

执行环境与周六相同。以下命令均在 **`io-sim` 目录**执行：

```bash
cargo run --quiet -- 2 '2,2,1;1'
```

按上述模型，标准输出必须为：

```text
trace: 0 0 1 IDLE 0
finish: 0=5 1=3
```

这表示任务 `0` 在时间 `[0,2)` 使用 CPU，随后阻塞至时间 `4`；任务 `1` 在 `[2,3)` 运行，`[3,4)` CPU 空闲，任务 `0` 在 `[4,5)` 完成。再运行：

```bash
cargo run --quiet -- 2 '2,2,1;5'
```

预期为：

```text
trace: 0 0 1 1 0 1 1 1
finish: 0=5 1=8
```

第二组专门验收时间 `4` 的入队顺序。无 I/O 的输入 `2 '5;3;1'` 应与周六相同输入的轨迹一致。非法阶段序列应以非零状态退出，不能卡在等待循环中。

### 4. 生成测试用例的完整 prompt

在 `io-sim` 目录打开编程助手，粘贴：

> 请以当前目录为 Rust 实验项目根目录，阅读已有代码和实验输入输出约定，只为 I/O 调度模拟器编写测试，不修改 `src/`，不替我实现调度逻辑。程序用 `cargo run --quiet -- <时间片> '<任务阶段;任务阶段>'` 执行；每个任务的阶段时长以逗号分隔，按 CPU、I/O、CPU 交替排列，所有任务在时间 0 到达。CPU 阶段按轮转时间片执行；I/O 阶段不占 CPU，结束时任务进入就绪队列；无就绪任务时输出 `IDLE`。同一时刻 I/O 完成与时间片到期时，唤醒任务先入队。输出格式是 `trace: ...` 和 `finish: ...`。请在 `tests/` 创建能用 `cargo test` 运行的集成测试，覆盖基本阻塞与唤醒、CPU 空闲、多任务 I/O 完成顺序、同时发生的唤醒与时间片到期、无 I/O 时与普通 RR 一致、单任务，以及 0 时间片、0 阶段、偶数个阶段、空任务和非数字输入。独立按规则推导断言，避免拿现有程序输出充当标准答案；错误输入检查非零退出及有意义的错误信息。最后列出运行命令和各测试的目的，不要提供实现代码。

### 5. 最原始的代码骨架

运行 `cargo new io-sim`，将 `src/main.rs` 改为：

```rust
use std::{env, process};

struct Report {
    trace: Vec<Option<usize>>, // None 表示该时间单位 CPU 空闲
    finish: Vec<u32>,
}

fn parse_input(args: &[String]) -> Result<(u32, Vec<Vec<u32>>), String> {
    // TODO: 解析时间片和各任务交替出现的 CPU/I/O 阶段
    todo!()
}

fn simulate(quantum: u32, jobs: &[Vec<u32>]) -> Report {
    // TODO: 维护就绪队列、被阻塞任务及其唤醒时间
    // TODO: 处理时间片结束、阶段结束、I/O 完成与 CPU 空闲
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (quantum, jobs) = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    let result = simulate(quantum, &jobs);
    let trace = result.trace.iter()
        .map(|id| id.map_or_else(|| "IDLE".to_string(), |id| id.to_string()))
        .collect::<Vec<_>>();
    let finish = result.finish.iter().enumerate()
        .map(|(id, time)| format!("{id}={time}")).collect::<Vec<_>>();
    println!("trace: {}", trace.join(" "));
    println!("finish: {}", finish.join(" "));
}
```