#[derive(Debug)]
pub struct Memory {
    ram: [u32; 256],
    rom: [u32; 256],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 256],  // ゼロ初期化
            rom: [0; 256],
        }
    }

    pub fn readRAM(&self, addr: u32) -> u32 {
        return self.ram[addr as usize];
    }

    pub fn writeRAM(&mut self, addr: u32, val: u32) {
        self.ram[addr as usize] = val;
    }

    pub fn readROM(&self, addr: u32) -> u32 {
        return self.rom[addr as usize];
    }
}