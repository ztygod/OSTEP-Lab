# OSTEp-Lab

用 Rust 练习《Operating Systems: Three Easy Pieces》中的调度实验。项目入口是 `src/main.rs`，运行后会显示实验菜单。

## 运行

在项目根目录执行：

```bash
cargo run
```

输入实验编号进行选择，输入 `0` 退出。菜单会标注每个实验的状态；选择尚未完成的实验只会显示提示，不会运行骨架代码。

## 实验

| 实验 | 代码 | 说明 | 当前状态 |
| --- | --- | --- | --- |
| 轮转调度（RR） | [`src/rr-sim/index.rs`](src/rr-sim/index.rs) | [`实验要求`](src/rr-sim/README.md) | 未完成 |
| I/O 阻塞与唤醒 | [`src/io-sim/index.rs`](src/io-sim/index.rs) | [`实验要求`](src/io-sim/README.md) | 未完成 |

两个实验目前保留骨架状态。完成某个实验后，在该模块中提供可从 `main.rs` 调用的 `pub fn run()` 入口，再把 `src/main.rs` 中对应实验的 `run: None` 改为 `run: Some(模块名::run)`。菜单就会将其标为“可运行”，选择后执行该入口。
