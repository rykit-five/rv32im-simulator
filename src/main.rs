pub mod register;

// #[macro_use]
// extern crate bitflags;

// use std::fmt;



// // Opcode

// // 2.4 Integer Computational Instructions
// const OP_IMM: u8    = 0x17;
// // 2.5 Control Transfer Instructions
// const JAL: u8       = 0x6F;
// const JALR: u8      = 0x67;
// const BRANCH: u8    = 0x63;
// // 2.6 Load and Store Instructions
// const LOAD: u8      = 0x03;
// const STORE: u8     = 0x23;
// // 2.7 Memory Model
// const MISC_MEM: u8  = 0x0F;
// // 2.8 Control and Status Register Instructions
// // 2.9 Environment Call and Breakpoints
// const SYSTEM: u8    = 0x73;


// // Funct

//     // LUI         = ,
//     // AUIPC       = ,
//     // ADDI        = 0x,
//     // SLTI        = 0x,
//     // SLTIU       = 0x,
//     // XORI        = 0x,
//     // ORI         = 0x,
//     // ANDI        = 0x,
//     // SLLI        = 0x,
//     // SRLI        = 0x,
//     // SRAI        = 0x,
//     // ADD         = 0x,
//     // SUB         = 0x,
//     // SLL         = 0x,
//     // SLT         = 0x,
//     // SLTU        = 0x,
//     // XOR         = 0x,
//     // SRL         = 0x,
//     // SRA         = 0x,
//     // OR          = 0x,
//     // AND         = 0x,
//     // FENCE       = 0x,
//     // FENCE_I     = 0x,
//     // ECALL       = 0x,
//     // EBREAK      = 0x,
//     // CSRRW       = 0x,
//     // CSRRS       = 0x,
//     // CSRRC       = 0x,
//     // CSRRWI      = 0x,
//     // CSRRSI      = 0x,
//     // CSRRCI      = 0x,
    
// bitflags! {
//     struct XTypeFlags: u32 {
//         const opcode    = 0x0000_007F;
//         const rd        = 0x0000_0F80;
//         const funct3    = 0x0000_7000;
//         const rs1       = 0x000F_8000;
//         const rs2       = 0x01F0_0000;
//     }
// }

// impl XTypeFlags {
//     pub fn intesection(&self, inst: u32) -> u32 {
//         inst & self.opcode
//     }
// }

// bitflags! {
//     struct RTypeFlags: u32 {
//         const funct5    = 0xFE00_0000;
//     }
// }

// bitflags! {
//     struct ITypeFlags: u32 {
//         const imm_11_0  = 0xFFF0_0000;
//     }
// }

// bitflags! {
//     struct STypeFlags: u32 {
//         const imm_4_0   = 0x0000_0F80;
//         const imm_11_5  = 0xFE00_0000;
//     }
// }

// bitflags! {
//     struct UTypeFlags: u32 {
//         const imm_31_12 = 0xFFFF_F000;
//     }
// }

// struct CPU {
//     pc: usize,
//     reg: [u32; 32],
//     ram: [u32; 256],
//     rom: [u32; 256],
// }

// impl CPU {
//     fn read_opcode(self, inst: u32) -> u8 {
//         // let pc = self.pc;
//         // let inst = self.rom[pc];
//         let opcode = (inst & XTypeFlags::opcode) as u8;
//         // let type_ = ((inst ))
//         return opcode;
//     }

//     fn read_funct3(self, inst: u32) -> u8 {
//         let funct3 = ((inst & 0x0000_7000) >> 12) as u8;
//         return funct3;
//     }

//     fn read_funct7(self, inst: u32) -> u8 {
//         let funct7 = ((inst & 0xFE00_0000) >> 25) as u8;
//         return funct7;
//     }

//     fn run(&mut self) {
//         loop {
//             let inst = self.rom[self.pc] as u32;
//             let opcode = self.read_opcode(inst);
//             match opcode {

//             }
            
//             let funct3 = self.read_funct3(inst);
//             self.pc += 4;

//             match opcode {
//                 OP_IMM      => {
                    
//                 },
//                 JAL         => ,
//                 JALR        => ,
//                 BRANCH      => ,
//                 LOAD        => ,
//                 STORE       => ,
//                 MISC_MEM    => ,
//                 SYSTEM      => ,
//                 _                   => todo!("opcode {:04x}", opcode),
//             }

//         }
//     }

//     fn add() {

//     }
// }

fn main() {

}
