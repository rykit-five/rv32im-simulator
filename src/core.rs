use std::convert::TryFrom;

// c.f., Table 26.1: RISC-V base opcode map
#[derive(Debug)]
pub enum Opcode {
    LOAD        = 0b0000011,
    LOAD_FP     = 0b0000111,
    MISC_MEM    = 0b0001111,
    OP_IMM      = 0b0010011,
    AUIPC       = 0b0010111,
    OP_IMM_32   = 0b0011011,

    STORE       = 0b0100011,
    STORE_FP    = 0b0100111,
    AMO         = 0b0101111,
    OP          = 0b0110011,
    LUI         = 0b0110111,
    OP_32       = 0b0111011,

    MADD        = 0b1000011,
    MSUB        = 0b1000111,
    MMSUB       = 0b1001011,
    NMADD       = 0b1001111,
    OP_FP       = 0b1010011,

    BRANCH      = 0b1100011,
    JALR        = 0b1100111,
    JAL         = 0b1101111,
    SYSTEM      = 0b1110011,
}

impl TryFrom<u32> for Opcode {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            val if val == Opcode::LOAD as u32       => Ok(Opcode::LOAD),
            val if val == Opcode::LOAD_FP as u32    => Ok(Opcode::LOAD_FP),
            val if val == Opcode::MISC_MEM as u32   => Ok(Opcode::MISC_MEM),
            val if val == Opcode::OP_IMM as u32     => Ok(Opcode::OP_IMM),
            val if val == Opcode::AUIPC as u32      => Ok(Opcode::AUIPC),
            val if val == Opcode::OP_IMM_32 as u32  => Ok(Opcode::OP_IMM_32),

            val if val == Opcode::STORE as u32      => Ok(Opcode::STORE),
            val if val == Opcode::STORE_FP as u32   => Ok(Opcode::STORE_FP),
            val if val == Opcode::AMO as u32        => Ok(Opcode::AMO),
            val if val == Opcode::OP as u32         => Ok(Opcode::OP),
            val if val == Opcode::LUI as u32        => Ok(Opcode::LUI),
            val if val == Opcode::OP_32 as u32      => Ok(Opcode::OP_32),

            val if val == Opcode::MADD as u32       => Ok(Opcode::MADD),
            val if val == Opcode::MSUB as u32       => Ok(Opcode::MSUB),
            val if val == Opcode::MMSUB as u32      => Ok(Opcode::MMSUB),
            val if val == Opcode::NMADD as u32      => Ok(Opcode::NMADD),
            val if val == Opcode::OP_FP as u32      => Ok(Opcode::OP_FP),

            val if val == Opcode::BRANCH as u32     => Ok(Opcode::BRANCH),
            val if val == Opcode::JALR as u32       => Ok(Opcode::JALR),
            val if val == Opcode::JAL as u32        => Ok(Opcode::JAL),
            val if val == Opcode::SYSTEM as u32     => Ok(Opcode::SYSTEM),
            _                                       => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Funct3OpImm {
    ADDI        = 0b000,
    SLLI        = 0b001,
    SLTI        = 0b010,
    SLTIU       = 0b011,
    XORI        = 0b100,
    SRLISRAI    = 0b101,
    ORI         = 0b110,
    ANDI        = 0b111,
}

impl TryFrom<u32> for Funct3OpImm {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            val if val == Funct3OpImm::ADDI as u32      => Ok(Funct3OpImm::ADDI),
            val if val == Funct3OpImm::SLLI as u32      => Ok(Funct3OpImm::SLLI),
            val if val == Funct3OpImm::SLTI as u32      => Ok(Funct3OpImm::SLTI),
            val if val == Funct3OpImm::SLTIU as u32     => Ok(Funct3OpImm::SLTIU),
            val if val == Funct3OpImm::XORI as u32      => Ok(Funct3OpImm::XORI),
            val if val == Funct3OpImm::SRLISRAI as u32  => Ok(Funct3OpImm::SRLISRAI),
            val if val == Funct3OpImm::ORI as u32       => Ok(Funct3OpImm::ORI),
            val if val == Funct3OpImm::ANDI as u32      => Ok(Funct3OpImm::ANDI), 
            _                                           => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Funct3Op {
    ADDSUB      = 0b000,
    SLL         = 0b001,
    SLT         = 0b010,
    SLTU        = 0b011,
    XOR         = 0b100,
    SRLSRA      = 0b101,
    OR          = 0b110,
    AND         = 0b111,
}

#[derive(Debug)]
pub enum Funct3Branch {
    BEQ         = 0b000,
    BNE         = 0b001,
    BLT         = 0b100,
    BGE         = 0b101,
    BLTU        = 0b110,
    BGEU        = 0b111,
}

#[derive(Debug)]
pub enum Funct3Load {
    LB          = 0b000,
    LH          = 0b001,
    LW          = 0b010,
    LBU         = 0b100,
    LHU         = 0b101,
}

impl TryFrom<u32> for Funct3Load {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            val if val == Funct3Load::LB as u32         => Ok(Funct3Load::LB),
            val if val == Funct3Load::LH as u32         => Ok(Funct3Load::LH),
            val if val == Funct3Load::LW as u32         => Ok(Funct3Load::LW),
            val if val == Funct3Load::LBU as u32        => Ok(Funct3Load::LBU),
            val if val == Funct3Load::LHU as u32        => Ok(Funct3Load::LHU),
            _                                           => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Funct3STORE {
    SB          = 0b000,
    SH          = 0b001,
    SW          = 0b010,
}

#[derive(Debug)]
pub enum OpLabel {
    
}

/*
#[derive(Debug)]
pub enum Funct3 {
    JALR        = 0b000,

    BEQ         = 0b000,
    BNE         = 0b001,
    BLT         = 0b100,
    BGE         = 0b101,
    BLTU        = 0b110,
    BGEU        = 0b111,

    LB          = 0b000,
    LH          = 0b001,
    LW          = 0b010,
    LBU         = 0b100,
    LHU         = 0b101,

    SB          = 0b000,
    SH          = 0b001,
    SW          = 0b010,

    ADDI        = 0b000,
    SLTI        = 0b010,
    SLTIU       = 0b011,
    XORI        = 0b100,
    ORI         = 0b110,
    ANDI        = 0b111,

    SLLI        = 0b001,
    SRLI        = 0b101,
    SRAI        = 0b101,

    ADD_SUB     = 0b000,
    SLL         = 0b001,
    SLT         = 0b010,
    SLTU        = 0b011,
    XOR         = 0b100,
    SRL         = 0b101,
    SRA         = 0b101,
    OR          = 0b110,
    AND         = 0b111,

    FNECE       = 0b000,
    FENCE_TSO   = 0b000,
    PAUSE       = 0b000,
    ECALL       = 0b000,
    EBREAK      = 0b000,
}

#[derive(Debug)]
pub enum Funct5 {

}
*/

