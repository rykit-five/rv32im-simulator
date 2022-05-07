use crate::decoder::{ITypeDecoder, OpcodeDecoder, RTypeDecoder, UTypeDecoder};
use crate::memory::Memory;
use crate::register::Register;

pub mod core;
pub mod register;
pub mod memory;
pub mod decoder;

#[derive(Debug)]
pub struct CPU {}

impl CPU {
    pub fn new() -> CPU {
        CPU {}
    }

    pub fn run(&self, reg: &mut Register, mem: &mut Memory) {
        loop {
            let inst: u32 = mem.readROM(reg.getPC());
            let op_dec: OpcodeDecoder = OpcodeDecoder::new(inst);

            // OpcodeからTypeを特定して他フィールドを読み出し
            match op_dec.opcode {
                Opcode::LOAD        => ,
                Opcode::LOAD_FP     => ,
                Opcode::MISC_MEM    => ,
                Opcode::OP_IMM      => {
                    let itype_dec: ITypeDecoder = ITypeDecoder::new(inst);
                    match itype_dec.funct3 {
                        Funct3OpImm::ADDI       => itype_dec.behaviorADDI(&mut reg, &mut mem);
                        Funct3OpImm::SLTI       => itype_dec.behaviorSLTI(&mut reg, &mut mem);
                        Funct3OpImm::SLLI       => itype_dec.behaviorSLLI(&mut reg, &mut mem);
                        Funct3OpImm::SLTIU      => itype_dec.behaviorSLTIU(&mut reg, &mut mem);
                        Funct3OpImm::XORI       => itype_dec.behaviorXORI(&mut reg, &mut mem);
                        Funct3OpImm::SRLISRAI   => {
                            match itype_dec.imm_11_5 {
                                0b000_0000      => itype_dec.behaviorSRLI(&mut reg, &mut mem);
                                0b000_0001      => itype_dec.behaviorSRAI(&mut reg, &mut mem);
                                _               => println!("Error"),
                            }
                        }
                        Funct3OpImm::ORI        => itype_dec.behaviorORI(&mut reg, &mut mem);
                        Funct3OpImm::ANDI       => itype_dec.behaviorANDI(&mut reg, &mut mem);
                        _                       => println!("Error"),
                    }
                },
                Opcode::AUIPC       => {
                    let utype_dec: UTypeDecoder = UTypeDecoder::new(inst);
                    utype_dec.behaviorAUIPC(&mut reg, &mut mem);
                },
                Opcode::OP_IMM_32   => ,
                Opcode::STORE       => ,
                Opcode::STORE_FP    => ,
                Opcode::AMO         => ,
                Opcode::OP          => {
                    let rtype_dec: RTypeDecoder = RTypeDecoder::new(inst);
                    match rtype_dec.funct3 {
                        Funct3Op::ADDSUB        => {
                            match rtype_dec.funct7 {
                                0b000_0000      => rtype_dec.behaviorADD(&mut reg, &mut mem);
                                0b010_0000      => rtype_dec.behaviorSUB(&mut reg, &mut mem);
                                _               => println!("Error"),
                            }
                        }
                        Funct3Op::SLT           => 
                        Funct3Op::SLTU          => 
                        Funct3Op::XOR           => 
                        Funct3Op::SRLSRA        => {
                            match rtype_dec.funct7 {
                                0b000_0000      => rtype_dec.behaviorSRL(&mut reg, &mut mem);
                                0b010_0000      => rtype_dec.behaviorSRA(&mut reg, &mut mem);
                                _               => println!("Error"),
                            }
                        }
                        Funct3Op::OR            => rtype_dec.behaviorOR(&mut reg, &mut mem);
                        Funct3Op::AND           => rtype_dec.behaviorAND(&mut reg, &mut mem);
                        _                       => println!("Error"),
                    }
                },
                Opcode::LUI         => {
                    let utype_dec: UTypeDecoder = UTypeDecoder::new(inst);
                    utype_dec.behaviorLUI(&mut reg, &mut mem);
                },
                Opcode::OP_32       => ,
                Opcode::MADD        => ,
                Opcode::MSUB        => ,
                Opcode::MMSUB       => ,
                Opcode::NMADD       => ,
                Opcode::OP_FP       => ,
                Opcode::BRANCH      => ,
                Opcode::JALR        => ,
                Opcode::JAL         => ,
                Opcode::SYSTEM      => ,
                _                   => println!("Error"),
            }
            
            // memo: いつでもPCを4インクリメント？
            reg.incPC();

        }
    }
}

fn main() {
    let mut reg: Register = register::Register::new();
    let mut mem: Memory = memory::Memory::new();

    let cpu: CPU = CPU::new();
    cpu.run(&mut reg, &mut mem);
}

