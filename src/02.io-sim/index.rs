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