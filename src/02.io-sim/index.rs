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