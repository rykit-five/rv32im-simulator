use crate::decoder::{
    OpcodeDecoder,
    RTypeDecoder,
    ITypeDecoder, 
    STypeDecoder,
    BTypeDecoder, 
    JTypeDecoder, 
    UTypeDecoder,
};
use crate::memory::Memory;
use crate::register::Register;
use crate::core::*;

pub mod core;
pub mod register;
pub mod memory;
pub mod decoder;
pub mod bittools;

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
            match Opcode::try_from(op_dec.opcode).unwrap() {
                Opcode::LOAD        => {
                    let itype_dec: ITypeDecoder = ITypeDecoder::new(inst);
                    match Funct3Load::try_from(itype_dec.funct3).unwrap() {
                        Funct3Load::LW          => itype_dec.behaviorLW(reg, mem),
                        Funct3Load::LH          => itype_dec.behaviorLH(reg, mem),
                        Funct3Load::LB          => itype_dec.behaviorLB(reg, mem),
                        Funct3Load::LHU         => itype_dec.behaviorLHU(reg, mem),
                        Funct3Load::LBU         => itype_dec.behaviorLBU(reg, mem),
                        _                       => println!("Decode error"),
                    }

                },
                Opcode::LOAD_FP     => ,
                Opcode::MISC_MEM    => ,
                Opcode::OP_IMM      => {
                    let itype_dec: ITypeDecoder = ITypeDecoder::new(inst);
                    match Funct3OpImm::try_from(itype_dec.funct3).unwrap() {
                        // todo: NOP命令を実装するか？
                        Funct3OpImm::ADDI       => itype_dec.behaviorADDI(reg, mem);
                        Funct3OpImm::SLTI       => itype_dec.behaviorSLTI(reg, mem);
                        Funct3OpImm::SLLI       => itype_dec.behaviorSLLI(reg, mem);
                        Funct3OpImm::SLTIU      => itype_dec.behaviorSLTIU(reg, mem);
                        Funct3OpImm::XORI       => itype_dec.behaviorXORI(reg, mem);
                        Funct3OpImm::SRLISRAI   => {
                            match itype_dec.imm_11_5 {
                                0b000_0000      => itype_dec.behaviorSRLI(reg, mem);
                                0b000_0001      => itype_dec.behaviorSRAI(reg, mem);
                                _               => println!("Decode error"),
                            }
                        }
                        Funct3OpImm::ORI        => itype_dec.behaviorORI(reg, mem);
                        Funct3OpImm::ANDI       => itype_dec.behaviorANDI(reg, mem);
                        _                       => println!("Decode error"),
                    }
                },
                Opcode::AUIPC       => {
                    let utype_dec: UTypeDecoder = UTypeDecoder::new(inst);
                    utype_dec.behaviorAUIPC(reg, mem);
                },
                Opcode::OP_IMM_32   => ,
                Opcode::STORE       => {
                    let stype_dec: STypeDecoder = STypeDecoder::new(inst);
                    match stype_dec.funct3 {
                        Funct3STORE::SW         => stype_dec.behaviorSW(reg, mem),
                        Funct3STORE::SH         => stype_dec.behaviorSH(reg, mem),
                        Funct3STORE::SB         => stype_dec.behaviorSB(reg, mem),
                        _                       => println!("Decode error"),
                    }
                },
                Opcode::STORE_FP    => ,
                Opcode::AMO         => ,
                Opcode::OP          => {
                    let rtype_dec: RTypeDecoder = RTypeDecoder::new(inst);
                    match rtype_dec.funct3 {
                        Funct3Op::ADDSUB        => {
                            match rtype_dec.funct7 {
                                0b000_0000      => rtype_dec.behaviorADD(reg, mem);
                                0b010_0000      => rtype_dec.behaviorSUB(reg, mem);
                                _               => println!("Decode error"),
                            }
                        }
                        Funct3Op::SLT           => ,
                        Funct3Op::SLTU          => ,
                        Funct3Op::XOR           => ,
                        Funct3Op::SRLSRA        => {
                            match rtype_dec.funct7 {
                                0b000_0000      => rtype_dec.behaviorSRL(reg, mem);
                                0b010_0000      => rtype_dec.behaviorSRA(reg, mem);
                                _               => println!("Decode error"),
                            }
                        }
                        Funct3Op::OR            => rtype_dec.behaviorOR(reg, mem);
                        Funct3Op::AND           => rtype_dec.behaviorAND(reg, mem);
                        _                       => println!("Decode error"),
                    }
                },
                Opcode::LUI         => {
                    let utype_dec: UTypeDecoder = UTypeDecoder::new(inst);
                    utype_dec.behaviorLUI(reg, mem);
                },
                Opcode::OP_32       => ,
                Opcode::MADD        => ,
                Opcode::MSUB        => ,
                Opcode::MMSUB       => ,
                Opcode::NMADD       => ,
                Opcode::OP_FP       => ,
                Opcode::BRANCH      => {
                    let btype_dec: BTypeDecoder = BTypeDecoder::new(inst);
                    match btype_dec.funct3 {
                        Funct3Branch::BEQ       => btype_dec.behaviorBEQ(reg, mem),
                        Funct3Branch::BNE       => btype_dec.behaviorBNE(reg, mem),
                        Funct3Branch::BLT       => btype_dec.behaviorBLT(reg, mem),
                        Funct3Branch::BGE       => btype_dec.behaviorBGE(reg, mem),
                        Funct3Branch::BLTU      => btype_dec.behaviorBLTU(reg, mem),
                        Funct3Branch::BGEU      => btype_dec.behaviorBGEU(reg, mem),
                        _                       => println!("Decode error"),
                    }
                },
                Opcode::JALR        => {
                    let itype_dec: ITypeDecoder = ITypeDecoder::new(inst);
                    itype_dec.behaviorJALR(reg, mem);
                },
                Opcode::JAL         => {
                    let jtype_dec: JTypeDecoder = JTypeDecoder::new(inst);
                    jtype_dec.behaviorJAL(reg, mem);
                },
                Opcode::SYSTEM      => ,
                _                   => println!("Decode error"),
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

