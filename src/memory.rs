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

    pub fn readRAM16Bit(&self, addr: u32) -> u16 {
        return (self.readRAM(addr) & 0x0000_FFFF) as u16;
    }

    pub fn readRAM8Bit(&self, addr: u32) -> u8 {
        return (self.readRAM(addr) & 0x0000_00FF) as u8;
    }

    pub fn writeRAM(&mut self, addr: u32, val: u32) {
        self.ram[addr as usize] = val;
    }

    pub fn writeRAM16Bit(&mut self, addr: u32, val: u16) {
        self.writeRAM(addr, val as u32);
    }

    pub fn writeRAM8Bit(&mut self, addr: u32, val: u8) {
        self.writeRAM(addr, val as u32);
    }

    pub fn readROM(&self, addr: u32) -> u32 {
        return self.rom[addr as usize];
    }
}