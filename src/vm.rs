pub struct VirtualMachine {
    regs: [u8; 4],
    buf: u32,
    count: u8,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            regs: [0; 4],
            buf: 0,
            count: 0,
        }
    }
    pub fn eval(&mut self, codes: &[u8], len: usize) {
        let mut i = 0;
        loop {
            if i == len {
                break;
            }
            let op = (codes[i] >> 5) & 0b111;
            if op == 0b111 {
                let prefix = (codes[i] >> 4) & 0b1;
                let dst_idx = ((codes[i] >> 2) & 0b11) as usize;
                let src_idx = (codes[i] & 0b11) as usize;
                if prefix == 0 {
                    break; // exit
                } else {
                    if self.regs[dst_idx] == self.regs[src_idx] {
                        i = self.buf as usize;
                        continue;
                    }
                }
            } else {
                self.execute(codes[i]);
            }
            i = i + 1;
        }
    }
    fn execute(&mut self, code: u8) {
        // [op 3bit] [dst 2bit] [prefix 1bit] [src 2bit]
        let op = (code >> 5) & 0b111; // 상위 3비트
        let dst = ((code >> 3) & 0b11) as usize; // 그 다음 2비트
        let prefix = (code >> 2) & 0b1; // 그 다음 1비트
        let src = code & 0b11; // 하위 2비트
        //for extened instruction set
        let extended = code & 0b00011111;
        let extended_op = extended & 0b00000011;
        let extened_prefix = (extended >> 4) & 0b1 != 0;
        let extened_imm = (extended >> 2) & 0b11;
        let src_val = if prefix == 0 {
            self.regs[src as usize]
        } else {
            src
        };

        match op {
            0b000 => {
                if extened_prefix {
                    match extended_op {
                        0b00 => {
                            if self.count < 16 {
                                self.buf |= (extened_imm as u32) << self.count;
                                self.count = self.count + 2;
                            } else {
                                println!("out of memory");
                            }
                        }
                        0b01 => {
                            if self.count >= 2 {
                                self.count -= 2;
                                let mask = !(0b11u32 << self.count);
                                self.buf &= mask;
                            }
                        }
                        0b10 => {
                            self.buf = 0;
                            self.count = 0;
                        }
                        0b11 => {
                            if let Some(c) = char::from_u32(self.buf) {
                                println!("{}", c);
                            } else {
                                println!("Invalid Unicode value");
                            }
                        }
                        _ => {}
                    }
                } else {
                    /*NOP*/
                }
            }
            0b001 => {
                // mov
                self.regs[dst] = src_val;
            }
            0b010 => {
                // add
                self.regs[dst] = self.regs[dst].wrapping_add(src_val);
            }
            0b011 => {
                // sub
                self.regs[dst] = self.regs[dst].wrapping_sub(src_val);
            }
            0b100 => {
                if src_val == 0 {
                    println!("Error: Division by zero!");
                } else {
                    self.regs[dst] = self.regs[dst].wrapping_div(src_val);
                }
            }
            0b101 => {
                self.regs[dst] = self.regs[dst].wrapping_mul(src_val);
            }
            0b110 => {
                self.regs[dst] = !(self.regs[dst] & src_val);
            }
            _ => println!("unknown opcode"),
        }
        println!(
            "register:\na: {}, b: {}, c: {}, d: {}",
            self.regs[0], self.regs[1], self.regs[2], self.regs[3]
        );
    }
}
