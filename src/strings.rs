pub enum StringAddr {
    Addr(usize),
    EOF,
    NoString,
}

pub fn strings(min_len: u32, data: &[u8]) {
    let mut curr_addr = 0usize;
    'outer: loop {
        let addr = 'addr: loop {
            let addr = match find_string(curr_addr, min_len, &data) {
                StringAddr::Addr(addr) => addr,
                StringAddr::EOF => break 'outer,
                StringAddr::NoString => {
                    curr_addr += 1;
                    continue 'addr;
                }
            };
            break addr;
        };
        let mut buffer = Vec::new();
        let mut offset = 0usize;
        'string: loop {
            if addr + offset == data.len() {
                break 'outer;
            }
            let c = data[addr + offset];
            match c {
                32...126 => buffer.push(c),
                0 => {
                    buffer.push(c);
                    break 'string;
                }
                _ => break 'string,
            }
            offset += 1;
        }
        let s = match String::from_utf8(buffer) {
            Ok(s) => s,
            Err(_e) => {
                break 'outer;
            }
        };
        println!("{}", s);
        curr_addr += offset + 1;
    }
}

fn find_string(start_addr: usize, min_len: u32, data: &[u8]) -> StringAddr {
    for i in 0..min_len as usize {
        if start_addr + i == data.len() {
            return StringAddr::EOF;
        }
        let c = data[start_addr + i];
        match c {
            32...126 => continue,
            _ => return StringAddr::NoString,
        }
    }
    StringAddr::Addr(start_addr)
}
