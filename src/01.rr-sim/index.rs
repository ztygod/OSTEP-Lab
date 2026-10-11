use std::{collections::VecDeque, env, process, vec};

struct Report {
    trace: Vec<usize>,
    finish: Vec<u32>,
    response: Vec<u32>,
}

fn parse_input(args: &[String]) -> Result<(u32, Vec<u32>), String> {
    // TODO: 解析并校验时间片与任务时长
    if args.len() != 3 {
        return Err("用法: rr-sim <时间片> <逗号分隔的任务时长>".into());
    }

    let quantum = args[1]
        .parse::<u32>()
        .map_err(|_| "时间片必须为正整数".to_string())?;
    if quantum == 0 {
        return Err("时间片必须为正整数".into());
    }

    let lengths = args[2]
        .split(",")
        .map(|text| {
            let length = text
                .parse::<u32>()
                .map_err(|_| format!("无效的任务时长: {text:?}"))?;
            if length == 0 {
                return Err("任务时长必须大于0".to_string());
            }
            Ok(length)
        })
        .collect::<Result<Vec<u32>, String>>()?;

    Ok((quantum, lengths))
}

fn simulate(quantum: u32, lengths: &[u32]) -> Report {
    assert!(quantum > 0, "时间片必须大于 0");

    let mut remaining = lengths.to_vec();
    let mut queue: VecDeque<usize> = (0..lengths.len()).collect();
    let mut report = Report {
        trace: Vec::new(),
        finish: vec![0; lengths.len()],
        response: vec![0; lengths.len()],
    };
    let mut time = 0;

    while let Some(id) = queue.pop_front() {
        if remaining[id] == lengths[id] {
            report.response[id] = time;
        }

        let run = quantum.min(remaining[id]);
        for _ in 0..run {
            report.trace.push(id);
        }
        time += run;
        remaining[id] -= run;

        if remaining[id] == 0 {
            report.finish[id] = time;
        } else {
            queue.push_back(id);
        }
    }

    report
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let (quantum, lengths) = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    let result = simulate(quantum, &lengths);
    let trace = result
        .trace
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>();
    let finish = result
        .finish
        .iter()
        .enumerate()
        .map(|(id, time)| format!("{id}={time}"))
        .collect::<Vec<_>>();
    let response = result
        .response
        .iter()
        .enumerate()
        .map(|(id, time)| format!("{id}={time}"))
        .collect::<Vec<_>>();
    println!("trace: {}", trace.join(" "));
    println!("finish: {}", finish.join(" "));
    println!("response: {}", response.join(" "));
}
