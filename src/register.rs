#[derive(Debug)]
pub struct Register {
    pc: u32,
    x: [u32; 32],
}

impl Register {
    pub fn new() -> Self {
        return Self {
            pc: 0,
            x: [0; 32],
            // Vec::with_capacity(32),  // Zeroing
        };
    }

    pub fn get_pc(&self) -> u32 {
        return self.pc;
    }

    pub fn set_pc(&mut self, target_pc: u32) {
        self.pc = target_pc;
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }

    pub fn get_x(&self, id: u8) -> u32 {
        return self.x[id as usize];
    }

    pub fn set_x(&mut self, id: u8, val: u32) {
        self.x[id as usize] = val;
    }
}

#[cfg(test)]
mod tests {
    use crate::register::*;

    #[test]
    fn test_setter_and_getter() {
        let mut reg = Register::new();

        assert_eq!(reg.get_pc(), 0);
        reg.set_pc(1);
        assert_eq!(reg.get_pc(), 1);
        reg.inc_pc();
        assert_eq!(reg.get_pc(), 2);

        assert_eq!(reg.get_x(0), 0);
        reg.set_x(0, 1);
        assert_eq!(reg.get_x(0), 1);
    }
}