#[derive(Debug)]
pub struct Memory {
    mem: [i32; 256],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: Default::default(),  // ゼロ初期化
        }
    }

    pub fn readMem(&self, addr: u32) -> i32 {
        return self.mem[addr as usize];
    }

    pub fn writeMem(&mut self, addr: u32, imm: i32) {  // memo: 多分mutを付けないと書き込みできない
        self.mem[addr as usize] = imm;
    }
}