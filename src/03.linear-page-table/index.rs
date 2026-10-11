use std::{env, process};

const PAGE_SIZE: usize = 16;
const VIRTUAL_SIZE: usize = 256;
const PHYSICAL_SIZE: usize = 128;

struct Input {
    table: [Option<usize>; VIRTUAL_SIZE / PAGE_SIZE],
    addresses: Vec<usize>,
}

enum Translation {
    Physical(usize),
    Fault,
    OutOfRange,
}

fn parse_input(args: &[String]) -> Result<Input, String> {
    // TODO: 解析映射和地址，并检查重复或越界的页号
    todo!()
}

fn translate(table: &[Option<usize>], virtual_address: usize) -> Translation {
    // TODO: 拆分 VPN 与偏移、查页表、组合物理地址
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    for address in input.addresses {
        let result = match translate(&input.table, address) {
            Translation::Physical(pa) => pa.to_string(),
            Translation::Fault => "FAULT".to_string(),
            Translation::OutOfRange => "OUT_OF_RANGE".to_string(),
        };
        println!("{address} -> {result}");
    }
}