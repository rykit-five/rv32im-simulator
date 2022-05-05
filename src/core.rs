// c.f., Table 26.1: RISC-V base opcode map
#[derive(Debug)]
enum Opcode {
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

#[derive(Debug)]
enum Funct3OpImm {
    ADDI        = 0b000,
    SLLI        = 0b001,
    SLTI        = 0b010,
    SLTIU       = 0b011,
    XORI        = 0b100,
    SRLISRAI    = 0b101,
    ORI         = 0b110,
    ANDI        = 0b111,
}

#[derive(Debug)]
enum Funct3Op {
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
enum OpLabel {
    
}

/*
#[derive(Debug)]
enum Funct3 {
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
enum Funct5 {

}
*/

