#[derive(Debug)]
enum Reg {
    ZERO,
    RA,
    SP,
    GP,
    TP,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

#[derive(Debug)]
pub struct Register {
    pc: u32,
    reg: [u32; 32],
}

impl Register {
    pub fn new() -> Self {
        return Self {
            pc: 0,
            reg: [0; 32],
            // Vec::with_capacity(32),  // Zeroing
        };
    }

    pub fn getPC(&self) -> u32 {
        return self.pc;
    }

    pub fn setPC(&mut self, target_pc: u32) {
        self.pc = target_pc;
    }

    pub fn incPC(&mut self) {
        // todo: オーバーフローを検出
        self.pc += 4;
    }

    pub fn getReg(&self, idx: u32) -> u32 {
        return self.reg[idx as usize];
    }

    pub fn setReg(&mut self, idx: u32, imm: u32) {
        self.reg[idx as usize] = imm;
    }
}

#[cfg(test)]
mod tests {
    use crate::register::*;

    #[test]
    fn test_setter_and_getter() {
        let mut reg = Register::new();

        assert_eq!(reg.getPC(), 0);
        reg.setPC(1);
        assert_eq!(reg.getPC(), 1);
        reg.incPC();
        assert_eq!(reg.getPC(), 2);

        assert_eq!(reg.getReg(Reg::T1), 0);
        reg.setReg(Reg::T1, 1);
        assert_eq!(reg.getReg(Reg::T1), 1);
    }
}