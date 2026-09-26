#[path = "io-sim/index.rs"]
#[allow(dead_code, unused_variables)] // 实验尚未完成，暂不调用骨架中的函数。
pub mod io_sim;
#[path = "rr-sim/index.rs"]
#[allow(dead_code, unused_variables)] // 实验尚未完成，暂不调用骨架中的函数。
pub mod rr_sim;

use std::io::{self, Write};

struct Experiment {
    name: &'static str,
    run: Option<fn()>,
}

// 实验完成并提供可调用入口后，将对应的 run 改为 Some(模块名::入口函数)。
const EXPERIMENTS: &[Experiment] = &[
    Experiment { name: "轮转调度（rr-sim）", run: None },
    Experiment { name: "I/O 阻塞与唤醒（io-sim）", run: None },
];

fn prompt(message: &str) -> io::Result<Option<String>> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        return Ok(None);
    }
    Ok(Some(input.trim().to_string()))
}

fn main() -> io::Result<()> {
    loop {
        println!("\n实验列表：");
        for (index, experiment) in EXPERIMENTS.iter().enumerate() {
            let status = if experiment.run.is_some() { "可运行" } else { "未完成" };
            println!("{}. {} [{status}]", index + 1, experiment.name);
        }
        println!("0. 退出");

        let Some(choice) = prompt("请输入编号: ")? else {
            break;
        };
        if choice == "0" {
            break;
        }
        match choice.parse::<usize>().ok().and_then(|index| index.checked_sub(1))
            .and_then(|index| EXPERIMENTS.get(index))
        {
            Some(Experiment { run: Some(run), .. }) => run(),
            Some(experiment) => println!("{} 尚未完成，暂不能运行。", experiment.name),
            None => eprintln!("无效的实验编号：{choice}"),
        }
    }
    Ok(())
}
