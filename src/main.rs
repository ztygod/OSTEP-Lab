#[path = "02.io-sim/index.rs"]
#[allow(dead_code, unused_variables)] // 实验尚未完成，暂不调用骨架中的函数。
pub mod io_sim;
#[path = "01.rr-sim/index.rs"]
pub mod rr_sim;

use std::{
    env,
    io::{self, Write},
    process::Command,
};

struct Experiment {
    name: &'static str,
    run: Option<fn() -> io::Result<()>>,
}

const EXPERIMENTS: &[Experiment] = &[
    Experiment {
        name: "轮转调度（rr-sim）",
        run: Some(run_rr_sim),
    },
    Experiment {
        name: "I/O 阻塞与唤醒（io-sim）",
        run: None,
    },
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

fn run_rr_sim() -> io::Result<()> {
    let Some(quantum) = prompt("请输入时间片: ")? else {
        return Ok(());
    };
    let Some(lengths) = prompt("请输入任务时长（逗号分隔）: ")? else {
        return Ok(());
    };

    // 复用命令行入口的解析与报错，同时让无效输入返回菜单。
    Command::new(env::current_exe()?)
        .arg(quantum)
        .arg(lengths)
        .status()?;
    Ok(())
}

fn main() -> io::Result<()> {
    if env::args_os().len() > 1 {
        rr_sim::main();
        return Ok(());
    }

    loop {
        println!("\n实验列表：");
        for (index, experiment) in EXPERIMENTS.iter().enumerate() {
            let status = if experiment.run.is_some() {
                "可运行"
            } else {
                "未完成"
            };
            println!("{}. {} [{status}]", index + 1, experiment.name);
        }
        println!("0. 退出");

        let Some(choice) = prompt("请输入编号: ")? else {
            break;
        };
        if choice == "0" {
            break;
        }
        match choice
            .parse::<usize>()
            .ok()
            .and_then(|index| index.checked_sub(1))
            .and_then(|index| EXPERIMENTS.get(index))
        {
            Some(Experiment { run: Some(run), .. }) => run()?,
            Some(experiment) => println!("{} 尚未完成，暂不能运行。", experiment.name),
            None => eprintln!("无效的实验编号：{choice}"),
        }
    }
    Ok(())
}
