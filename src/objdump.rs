use crate::elfheader::ElfHeader;
use capstone::prelude::*;

pub fn objdump(data: &[u8]) {
    let elf = ElfHeader::new(&data).unwrap();
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .unwrap();

    let instrs = cs
        .disasm_all(data, elf.entry)
        .expect("Failed to dissassemble the program");

    println!("buffer len {}", data.len());
    println!("Found {} instructions", instrs.len());
    for i in instrs.iter() {
        println!("{}", i);
    }
}
