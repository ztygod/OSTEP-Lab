use std::{env, process};

const PAGE_SIZE: usize = 16;
const VIRTUAL_SIZE: usize = 256;
const PHYSICAL_SIZE: usize = 128;

struct TwoLevelTable {
    directory: [Option<[Option<usize>; 4]>; 4],
}

enum Translation {
    Physical { address: usize, pde: usize, pte: usize },
    PdeFault,
    PteFault,
    OutOfRange,
}

fn parse_input(args: &[String]) -> Result<(Vec<(usize, usize)>, Vec<usize>), String> {
    // TODO: 解析 VPN:PFN 映射和待翻译的地址
    todo!()
}

impl TwoLevelTable {
    fn new() -> Self {
        Self { directory: [None; 4] }
    }

    fn map(&mut self, vpn: usize, pfn: usize) -> Result<(), String> {
        // TODO: 拆分 VPN；按需创建二级表；检查重复与越界映射
        todo!()
    }

    fn translate(&self, virtual_address: usize) -> Translation {
        // TODO: 依次检查地址范围、PDE、PTE，最后组合物理地址
        todo!()
    }

    fn entry_count(&self) -> usize {
        // TODO: 目录项数 + 已创建二级表的表项数
        todo!()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mappings, addresses) = parse_input(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });
    let mut table = TwoLevelTable::new();
    for (vpn, pfn) in mappings {
        table.map(vpn, pfn).unwrap_or_else(|error| {
            eprintln!("error: {error}");
            process::exit(1);
        });
    }
    for va in addresses {
        match table.translate(va) {
            Translation::Physical { address, pde, pte } =>
                println!("{va} -> {address} (pde={pde},pte={pte})"),
            Translation::PdeFault => println!("{va} -> PDE_FAULT"),
            Translation::PteFault => println!("{va} -> PTE_FAULT"),
            Translation::OutOfRange => println!("{va} -> OUT_OF_RANGE"),
        }
    }
    println!("entries: {}", table.entry_count());
}