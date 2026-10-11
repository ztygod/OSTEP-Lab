use std::{
    collections::{BTreeMap, VecDeque},
    env, process,
};

struct Report {
    trace: Vec<Option<usize>>, // None 表示该时间单位 CPU 空闲
    finish: Vec<u32>,
}

fn parse_input(args: &[String]) -> Result<(u32, Vec<Vec<u32>>), String> {
    // TODO: 解析时间片和各任务交替出现的 CPU/I/O 阶段
    let quantum = args[1]
        .parse::<u32>()
        .map_err(|_| "时间片必须为数字".to_string())?;
    if quantum <= 0 {
        return Err("时间片不允许为0".to_string());
    }

    let tasks: Vec<&str> = args[2].split(";").collect();
    let jobs = tasks
        .iter()
        .map(|&item| {
            let phases = item
                .split(",")
                .map(|text| {
                    let duration = text
                        .parse::<u32>()
                        .map_err(|_| format!("阶段时长必须为数字:{text:?}"))?;

                    Ok(duration)
                })
                .collect::<Result<Vec<u32>, String>>()?;

            if phases.len() / 2 == 0 {
                return Err("任务必须以 CPU 阶段结束".to_string());
            }
            Ok(phases)
        })
        .collect::<Result<Vec<Vec<u32>>, String>>()?;

    Ok((quantum, jobs))
}

fn simulate(quantum: u32, jobs: &[Vec<u32>]) -> Report {
    assert!(quantum > 0, "时间片必须大于 0");
    assert!(
        jobs.iter()
            .all(|phases| phases.len() % 2 == 1 && phases.iter().all(|&duration| duration > 0)),
        "任务必须由正时长的 CPU/I/O 阶段组成，并以 CPU 阶段结束"
    );

    let mut report = Report {
        trace: Vec::new(),
        finish: vec![0; jobs.len()],
    };
    let mut ready: VecDeque<usize> = (0..jobs.len()).collect();
    let mut phase_index = vec![0; jobs.len()];
    let mut cpu_remaining: Vec<u32> = jobs.iter().map(|phases| phases[0]).collect();
    let mut wakeups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    let mut time = 0;
    let mut completed = 0;

    while completed < jobs.len() {
        if let Some(mut ids) = wakeups.remove(&time) {
            ids.sort_unstable();
            ready.extend(ids);
        }

        let Some(id) = ready.pop_front() else {
            let (&next_wakeup, _) = wakeups.first_key_value().expect("未完成任务必须能够唤醒");
            report
                .trace
                .extend(std::iter::repeat_n(None, (next_wakeup - time) as usize));
            time = next_wakeup;
            continue;
        };

        let run = quantum.min(cpu_remaining[id]);
        for _ in 0..run {
            report.trace.push(Some(id));
            time += 1;

            // 唤醒任务先入队，再处理当前任务的时间片到期。
            if let Some(mut ids) = wakeups.remove(&time) {
                ids.sort_unstable();
                ready.extend(ids);
            }
        }
        cpu_remaining[id] -= run;

        if cpu_remaining[id] > 0 {
            ready.push_back(id);
            continue;
        }

        let io_index = phase_index[id] + 1;
        if io_index == jobs[id].len() {
            report.finish[id] = time;
            completed += 1;
        } else {
            phase_index[id] = io_index + 1;
            cpu_remaining[id] = jobs[id][phase_index[id]];
            wakeups
                .entry(time + jobs[id][io_index])
                .or_default()
                .push(id);
        }
    }

    report
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (quantum, jobs) = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    let result = simulate(quantum, &jobs);
    let trace = result
        .trace
        .iter()
        .map(|id| id.map_or_else(|| "IDLE".to_string(), |id| id.to_string()))
        .collect::<Vec<_>>();
    let finish = result
        .finish
        .iter()
        .enumerate()
        .map(|(id, time)| format!("{id}={time}"))
        .collect::<Vec<_>>();
    println!("trace: {}", trace.join(" "));
    println!("finish: {}", finish.join(" "));
}
