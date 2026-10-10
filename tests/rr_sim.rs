use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_OSTEp-Lab"))
        .args(args)
        .output()
        .expect("应能启动实验程序")
}

fn assert_success(args: &[&str], expected: [&str; 3]) {
    let output = run(args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("标准输出应为 UTF-8");
    assert_eq!(stdout.lines().collect::<Vec<_>>(), expected);
    assert!(output.stderr.is_empty());
}

fn assert_invalid(args: &[&str], hint: &str) {
    let output = run(args);
    assert!(!output.status.success(), "无效输入应以非零状态退出");
    assert!(output.stdout.is_empty(), "无效输入不应输出调度结果");
    let stderr = String::from_utf8(output.stderr).expect("错误信息应为 UTF-8");
    assert!(
        stderr.contains(hint),
        "错误信息应包含 {hint:?}，实际为 {stderr:?}"
    );
}

#[test]
fn multiple_jobs_rotate_and_keep_original_ids() {
    assert_success(
        &["2", "5,3,1"],
        [
            "trace: 0 0 1 1 2 0 0 1 0",
            "finish: 0=9 1=8 2=5",
            "response: 0=0 1=2 2=4",
        ],
    );
}

#[test]
fn jobs_finishing_exactly_at_quantum_boundary_do_not_reenter_queue() {
    assert_success(
        &["2", "2,4,2"],
        [
            "trace: 0 0 1 1 2 2 1 1",
            "finish: 0=2 1=8 2=6",
            "response: 0=0 1=2 2=4",
        ],
    );
}

#[test]
fn single_job_runs_until_completion() {
    assert_success(
        &["2", "3"],
        ["trace: 0 0 0", "finish: 0=3", "response: 0=0"],
    );
}

#[test]
fn large_quantum_finishes_each_job_on_first_turn() {
    assert_success(
        &["10", "2,1,3"],
        [
            "trace: 0 0 1 2 2 2",
            "finish: 0=2 1=3 2=6",
            "response: 0=0 1=2 2=3",
        ],
    );
}

#[test]
fn zero_quantum_is_rejected() {
    assert_invalid(&["0", "5,3"], "时间片");
}

#[test]
fn empty_job_list_is_rejected() {
    assert_invalid(&["2", ""], "任务");
}

#[test]
fn zero_length_job_is_rejected() {
    assert_invalid(&["2", "5,0,3"], "任务时长");
}

#[test]
fn nonnumeric_quantum_is_rejected() {
    assert_invalid(&["abc", "5,3"], "时间片");
}

#[test]
fn nonnumeric_job_length_is_rejected() {
    assert_invalid(&["2", "5,x"], "任务时长");
}

#[test]
fn menu_can_run_rr_sim_and_return_to_selection() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_OSTEp-Lab"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("应能启动菜单");
    child
        .stdin
        .take()
        .expect("应有标准输入")
        .write_all(b"1\n2\n3\n0\n")
        .expect("应能输入菜单选项和实验参数");

    let output = child.wait_with_output().expect("菜单应正常退出");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("标准输出应为 UTF-8");
    assert!(stdout.contains("轮转调度（rr-sim） [可运行]"));
    assert!(stdout.contains("trace: 0 0 0"));
    assert!(stdout.contains("finish: 0=3"));
    assert!(stdout.contains("response: 0=0"));
    assert_eq!(stdout.matches("实验列表：").count(), 2);
}

#[test]
fn menu_returns_to_selection_after_invalid_rr_input() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_OSTEp-Lab"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("应能启动菜单");
    child
        .stdin
        .take()
        .expect("应有标准输入")
        .write_all(b"1\n0\n5,3\n0\n")
        .expect("应能输入无效参数并退出菜单");

    let output = child.wait_with_output().expect("菜单应正常退出");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("标准输出应为 UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("错误信息应为 UTF-8");
    assert_eq!(stdout.matches("实验列表：").count(), 2);
    assert!(stderr.contains("时间片"));
}
