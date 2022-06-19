use crate::memory::Memory;
use crate::register::Register;
use crate::bittools::{
    signExtend,
    zeroExtend,
}

// pub mod register;
// pub mod memory;
// pub mod bittools;

// #[derive(Debug)]
// pub struct BitFields {
//     OPCODE  : OpcodeBitFields,
//     RTYPE   : RTypeBitFields,
//     ITYPE   : ITypeBitFields,
//     STYPE   : STypeBitFields,
//     BTYPE   : BTypeBitFields,
//     UTYPE   : UTypeBitFields,
//     JTYPE   : JTypeBitFields,
// }
//
// impl BitFields {
//     pub fn new() -> BitFields {
//         BitFields {
//             OPCODE  : OpcodeBitFields::new(),
//             RTYPE   : RTypeBitFields::new(),
//             ITYPE   : ITypeBitFields::new(),
//             STYPE   : STypeBitFields::new(),
//             BTYPE   : BTypeBitFields::new(),
//             UTYPE   : UTypeBitFields::new(),
//             JTYPE   : JTypeBitFields::new(),
//         }
//     }
// }

// Opcode

#[derive(Debug)]
pub struct OpcodeBitFields {
    opcode      : u32,
}

impl OpcodeBitFields {
    pub fn new() -> OpcodeBitFields {
        OpcodeBitFields {
            opcode      : 0x0000_007F,
        }
    }
}

#[derive(Debug)]
pub struct OpcodeDecoder {
    pub opcode      : u32,
}

impl OpcodeDecoder {
    pub fn new(inst: u32) -> OpcodeDecoder {
        let bf: OpcodeBitFields = OpcodeBitFields::new();
        OpcodeDecoder {
            opcode      : (inst & bf.opcode),
        }
    }
}

// R-Type

#[derive(Debug)]
pub struct RTypeBitFields {
    rd          : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    funct7      : u32,
}

impl RTypeBitFields {
    pub fn new() -> RTypeBitFields {
        RTypeBitFields {
            rd          : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            funct7      : 0xFE00_0000,
        }
    }
}

#[derive(Debug)]
pub struct RTypeDecoder {
    pub rd          : u32,
    pub funct3      : u32,
    pub rs1         : u32,
    pub rs2         : u32,
    pub funct7      : u32,
}

impl RTypeDecoder {
    pub fn new(inst: u32) -> RTypeDecoder {
        let bf: RTypeBitFields = RTypeBitFields::new();
        RTypeDecoder {
            rd          : (inst & bf.rd) >> 7,
            funct3      : (inst & bf.funct3) >> 12,
            rs1         : (inst & bf.rs1) >> 15,
            rs2         : (inst & bf.rs2) >> 20,
            funct7      : (inst & bf.funct7) >> 25,
        }
    }

    // ADD performs the addition of rs1 and rs2. SUB performs the subtraction of rs2 from rs1.
    // Overflows are ignored and the low XLEN bits of results are written to the destination rd.
    pub fn behaviorADD(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) + r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    pub fn behaviorSUB(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) - r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    // SLT and SLTU perform signed and unsigned compares respectively, writing 1 to rd if rs1 < rs2,
    // 0 otherwise. Note, SLTU rd, x0, rs2 sets rd to 1 if rs2 is not equal to zero, otherwise sets
    // rd to zero (assembler pseudoinstruction SNEZ rd, rs).
    pub fn behaviorSLT(&self, r: &mut register::Register, m: &mut memory::Memory) {
        if (r.getReg(self.rs1) as i32) < (r.getReg(self.rs2) as i32) {
            r.setReg(self.rd, 1);
        } else {
            r.setReg(self.rd, 0);
        }
    }

    pub fn behaviorSLTU(&self, r: &mut register::Register, m: &mut memory::Memory) {
        if r.getReg(self.rs1) < r.getReg(self.rs2) {
            r.setReg(self.rd, 1);
        } else {
            r.setReg(self.rd, 0);
        }
    }

    // AND, OR, and XOR perform bitwise logical operations.
    pub fn behaviorAND(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) & r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    pub fn behaviorOR(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) | r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    pub fn behaviorXOR(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) ^ r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    // SLL, SRL, and SRA perform logical left, logical right, and arithmetic right shifts on the
    // value in register rs1 by the shift amount held in the lower 5 bits of register rs2.
    pub fn behaviorSLL(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) << r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    pub fn behaviorSRL(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) >> r.getReg(self.rs2);
        r.setReg(self.rd, t1);
    }

    pub fn behaviorSRA(&self, r: &mut register::Register, m: &mut memory::Memory) {
        // todo: 要検証
        let sign_bit: u32 = r.getReg(self.rs1) >> 4;
        let s_pos: u32 = if r.getReg(self.rs2) >= 5 { 0 } else { 4 - r.getReg(self.rs2) };
        let e_pos: u32 = 32;
        let mut vacated_upper_bits: u32 = 0;
        for i in s_pos..e_pos {
            vacated_upper_bits |= sign_bit << i;
        }

        let t1: u32 = vacated_upper_bits | (r.getReg(self.rs1) >> r.getReg(self.rs2));
        r.setReg(self.rd, t1);
    }

    pub fn behavior(&self, r: &mut register::Register, m: &mut memory::Memory) {

    }
}

// I-Type

#[derive(Debug)]
pub struct ITypeBitFields {
    rd          : u32,
    funct3      : u32,
    rs1         : u32,
    imm_11_0    : u32,
    imm_4_0     : u32,
    imm_11_5    : u32,
}

impl ITypeBitFields {
    pub fn new() -> ITypeBitFields {
        ITypeBitFields {
            rd          : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            imm_11_0    : 0xFFF0_0000,
            imm_4_0     : 0x01F0_0000,
            imm_11_5    : 0xFE00_0000,
        }
    }
}

#[derive(Debug)]
pub struct ITypeDecoder {
    pub rd          : u32,
    pub funct3      : u32,
    pub rs1         : u32,
    pub imm_11_0    : u32,
    pub imm_4_0     : u32,
    pub imm_11_5    : u32,
}

impl ITypeDecoder {
    pub fn new(inst: u32) -> ITypeDecoder {
        let bf: ITypeBitFields = ITypeBitFields::new();
        ITypeDecoder {
            rd          : (inst & bf.rd) >> 7,
            funct3      : (inst & bf.funct3) >> 12,
            rs1         : (inst & bf.rs1) >> 15,
            imm_11_0    : (inst & bf.imm_11_0) >> 20,
            imm_4_0     : (inst & bf.imm_4_0) >> 20,
            imm_11_5    : (inst & bf.imm_11_5) >> 25,
        }
    }

    // ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored
    // and the result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to
    // implement the MV rd, rs1 assembler pseudoinpub struction.
    pub fn behaviorADDI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = r.getReg(self.rs1) + self.imm_11_0;
        r.setReg(self.rd, t);
    }

    // SLTI (set less than immediate) places the value 1 in register rd if register rs1 is less than
    // the signextended immediate when both are treated as signed numbers, else 0 is written to rd.
    // SLTIU is similar but compares the values as unsigned numbers (i.e., the immediate is first
    // sign-extended to XLEN bits then treated as an unsigned number). Note, SLTIU rd, rs1, 1 sets
    // rd to 1 if rs1 equals zero, otherwise sets rd to 0 (assembler pseudoinpub struction SEQZ rd,
    // rs).
    pub fn behaviorSLTI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        if (r.getReg(self.rs1) as i32) < (self.imm_11_0 as i32) {
            r.setReg(self.rd, 1);
        } else {
            r.setReg(self.rd, 0);
        }
    }

    pub fn behaviorSLTIU(&self, r: &mut register::Register, m: &mut memory::Memory) {
        // todo: unsigned intに直したい
        if r.getReg(self.rs1) < self.imm_11_0 {
            r.setReg(self.rd, 1);
        } else {
            r.setReg(self.rd, 0);
        }
    }

    // ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1
    // and the sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1
    // performs a bitwise logical inversion of register rs1 (assembler pseudoinstruction NOT rd,
    // rs).
    pub fn behaviorXORI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) ^ self.imm_11_0;
        r.setReg(self.rd, t1);
    }

    pub fn behaviorORI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) | self.imm_11_0;
        r.setReg(self.rd, t1);
    }

    pub fn behaviorANDI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) & self.imm_11_0;
        r.setReg(self.rd, t1);
    }

    // Shifts by a constant are encoded as a specialization of the I-type format. The operand to be
    // shifted is in rs1, and the shift amount is encoded in the lower 5 bits of the I-immediate
    // field. The right shift type is encoded in bit 30. SLLI is a logical left shift (zeros are
    // shifted into the lower bits); SRLI is a logical right shift (zeros are shifted into the upper
    // bits); and SRAI is an arithmetic right shift (the original sign bit is copied into the
    // vacated upper bits).
    pub fn behaviorSLLI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) << self.imm_4_0;
        r.setReg(self.rd, t1);
    }

    pub fn behaviorSRLI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) >> self.imm_4_0;
        r.setReg(self.rd, t1);
    }

    // todo: 動作検証が必須
    pub fn behaviorSRAI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let sign_bit: u32 = r.getReg(self.rs1) >> 4;
        let s_pos: u32 = if self.imm_4_0 >= 5 { 0 } else { 4 - self.imm_4_0 };
        let e_pos: u32 = 32;
        let mut vacated_upper_bits: u32 = 0;
        for i in s_pos..e_pos {
            vacated_upper_bits |= sign_bit << i;
        }

        let t1: u32 = vacated_upper_bits | r.getReg(self.rs1) >> self.imm_4_0;
        r.setReg(self.rd, t1);
    }

    // The indirect jump instruction JALR (jump and link register) uses the I-type encoding. The
    // target address is obtained by adding the sign-extended 12-bit I-immediate to the register
    // rs1, then setting the least-significant bit of the result to zero. The address of the
    // instruction following the jump (pc+4) is written to register rd. Register x0 can be used as
    // the destination if the result is not required.
    pub fn behaviorJALR(&self, r: &mut register::Register, m: &mut memory::Memory) {
        r.setReg(self.rd, r.getPC + 4);
        let t1: u32 = bittools::signExtend(self.imm_11_0, 12) + r.getReg(self.rs1);
        let t2: u32 = (t1 >> 1) << 1;
        r.setReg(self.rd, t2);
    }

    // Load and store instructions transfer a value between the registers and memory. Loads are
    // encoded in the I-type format and stores are S-type. The effective address is obtained by
    // adding register rs1 to the sign-extended 12-bit offset. Loads copy a value from memory to
    // register rd. Stores copy the value in register rs2 to memory.

    // The LW instruction loads a 32-bit value from memory into rd. LH loads a 16-bit value from
    // memory, then sign-extends to 32-bits before storing in rd. LHU loads a 16-bit value from
    // memory but then zero extends to 32-bits before storing in rd. LB and LBU are defined
    // analogously for 8-bit values.
    pub fn behaviorLW(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) + bitools::signExtend(self.imm_11_0, 12);
        let t2: u32 = m.readRAM(t1);
        r.setReg(self.rd, t2);
    }

    pub fn behaviorLH(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) + bitools::signExtend(self.imm_11_0, 12);
        let t2: u16 = bittools::signExtend(m.readRAM16Bit(t1));
        r.setReg(self.rd, t2 as u32);
    }

    pub fn behaviorLHU(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) + bitools::signExtend(self.imm_11_0, 12);
        let t2: u16 = bittools::zeroExtend(m.readRAM16Bit(t1));
        r.setReg(self.rd, t2 as u32);
    }

    pub fn behaviorLB(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) + bitools::signExtend(self.imm_11_0, 12);
        let t2: u8 = bittools::signExtend(m.readRAM8Bit(t1));
        r.setReg(self.rd, t2 as u32);
    }

    pub fn behaviorLBU(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = r.getReg(self.rs1) + bitools::signExtend(self.imm_11_0, 12);
        let t2: u8 = bittools::zeroExtend(m.readRAM8Bit(t1));
        r.setReg(self.rd, t2 as u32);
    }

}

// S-Type

#[derive(Debug)]
pub struct STypeBitFields {
    imm_4_0     : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    imm_11_5    : u32,
}

impl STypeBitFields {
    pub fn new() -> STypeBitFields {
        STypeBitFields {
            imm_4_0     : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            imm_11_5    : 0xFE00_0000,
        }
    }
}

#[derive(Debug)]
pub struct STypeDecoder {
    pub imm_4_0     : u32,
    pub funct3      : u32,
    pub rs1         : u32,
    pub rs2         : u32,
    pub imm_11_5    : u32,
}

impl STypeDecoder {
    pub fn new(inst: u32) -> STypeDecoder {
        let bf: STypeBitFields = STypeBitFields::new();
        STypeDecoder {
            imm_4_0     : (inst & bf.imm_4_0) >> 7,
            funct3      : (inst & bf.funct3) >> 12,
            rs1         : (inst & bf.rs1) >> 15,
            rs2         : (inst & bf.rs2) >> 20,
            imm_11_5    : (inst & bf.imm_11_5) >> 25,
        }
    }

    // Load and store instructions transfer a value between the registers and memory. Loads are
    // encoded in the I-type format and stores are S-type. The effective address is obtained by
    // adding register rs1 to the sign-extended 12-bit offset. Loads copy a value from memory to
    // register rd. Stores copy the value in register rs2 to memory.

    // The SW, SH, and SB instructions store 32-bit, 16-bit, and 8-bit values from the low bits of
    // register rs2 to memory.
    pub fn behaviorSW(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_11_5 << 4 | self.imm_4_0;
        let t2: u32 = r.getReg(self.rs1) + bitools::signExtend(t1, 12);
        m.writeRAM(t2, r.getReg(self.rs2));
    }

    pub fn behaviorSH(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_11_5 << 4 | self.imm_4_0;
        let t2: u32 = r.getReg(self.rs1) + bitools::signExtend(t1, 12);
        m.writeRAM(t2, r.getReg(self.rs2) as u16);
    }

    pub fn behaviorSB(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_11_5 << 4 | self.imm_4_0;
        let t2: u32 = r.getReg(self.rs1) + bitools::signExtend(t1, 12);
        m.writeRAM(t2, r.getReg(self.rs2) as u8);
    }

    pub fn behavior(&self, r: &mut register::Register, m: &mut memory::Memory) {

    }

}

// B-Type

#[derive(Debug)]
pub struct BTypeBitFields {
    imm_11      : u32,
    imm_4_1     : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    imm_10_5    : u32,
    imm_12      : u32,
}

impl BTypeBitFields {
    pub fn new() -> BTypeBitFields {
        BTypeBitFields {
            imm_11      : 0x0000_0080,
            imm_4_1     : 0x0000_0F00,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            imm_10_5    : 0x7C00_0000,
            imm_12      : 0x8000_0000,
        }
    }

    // All branch instructions use the B-type instruction format. The 12-bit B-immediate encodes
    // signed offsets in multiples of 2 bytes. The offset is sign-extended and added to the address
    // of the branch instruction to give the target address. The conditional branch range is ±4 KiB.
    // Branch instructions compare two registers. BEQ and BNE take the branch if registers rs1 and
    // rs2 are equal or unequal respectively. BLT and BLTU take the branch if rs1 is less than rs2,
    // using signed and unsigned comparison respectively. BGE and BGEU take the branch if rs1 is
    // greater than or equal to rs2, using signed and unsigned comparison respectively. Note, BGT,
    // BGTU, BLE, and BLEU can be synthesized by reversing the operands to BLT, BLTU, BGE, and BGEU,
    // respectively.
    pub fn behaviorBEQ(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_12 << 11
            | self.imm_11 << 10
            | self.imm_10_5 << 4
            | self.imm_4_1;
        let t2: u32 = bittools::signExtend(t1, 12) + r.getPC();
        if r.getReg(self.rs1) == r.getReg(self.rs2) {
            r.setPC(t2);
        }
    }

    pub fn behaviorBNE(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_12 << 11
            | self.imm_11 << 10
            | self.imm_10_5 << 4
            | self.imm_4_1;
        let t2: u32 = bittools::signExtend(t1, 12) + r.getPC();
        if r.getReg(self.rs1) != r.getReg(self.rs2) {
            r.setPC(t2);
        }
    }

    pub fn behaviorBLT(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_12 << 11
            | self.imm_11 << 10
            | self.imm_10_5 << 4
            | self.imm_4_1;
        let t2: u32 = bittools::signExtend(t1, 12) + r.getPC();
        if (r.getReg(self.rs1) as i32) < (r.getReg(self.rs2) as i32) {
            r.setPC(t2);
        }
    }

    pub fn behaviorBLTU(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_12 << 11
            | self.imm_11 << 10
            | self.imm_10_5 << 4
            | self.imm_4_1;
        let t2: u32 = bittools::signExtend(t1, 12) + r.getPC();
        if r.getReg(self.rs1) < r.getReg(self.rs2) {
            r.setPC(t2);
        }
    }

    pub fn behaviorBGE(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_12 << 11
            | self.imm_11 << 10
            | self.imm_10_5 << 4
            | self.imm_4_1;
        let t2: u32 = bittools::signExtend(t1, 12) + r.getPC();
        if (r.getReg(self.rs1) as i32) >= (r.getReg(self.rs2) as i32) {
            r.setPC(t2);
        }
    }

    pub fn behaviorBGEU(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_12 << 11
            | self.imm_11 << 10
            | self.imm_10_5 << 4
            | self.imm_4_1;
        let t2: u32 = bittools::signExtend(t1, 12) + r.getPC();
        if r.getReg(self.rs1) >= r.getReg(self.rs2) {
            r.setPC(t2);
        }
    }

    pub fn behavior(&self, r: &mut register::Register, m: &mut memory::Memory) {

    }
}

#[derive(Debug)]
pub struct BTypeDecoder {
    pub imm_11      : u32,
    pub imm_4_1     : u32,
    pub funct3      : u32,
    pub rs1         : u32,
    pub rs2         : u32,
    pub imm_10_5    : u32,
    pub imm_12      : u32,
}

impl BTypeDecoder {
    pub fn new(inst: u32) -> BTypeDecoder {
        let bf: BTypeBitFields = BTypeBitFields::new();
        BTypeDecoder {
            imm_11      : (inst & bf.imm_11) >> 7,
            imm_4_1     : (inst & bf.imm_4_1) >> 8,
            funct3      : (inst & bf.funct3) >> 12,
            rs1         : (inst & bf.rs1) >> 15,
            rs2         : (inst & bf.rs2) >> 20,
            imm_10_5    : (inst & bf.imm_10_5) >> 25,
            imm_12      : (inst & bf.imm_12) >> 31,
        }
    }
}

// U-Type

#[derive(Debug)]
pub struct UTypeBitFields {
    rd          : u32,
    imm_31_12   : u32,
}

impl UTypeBitFields {
    pub fn new() -> UTypeBitFields {
        UTypeBitFields {
            rd          : 0x0000_0F80,
            imm_31_12   : 0xFFFF_F000,
        }
    }

    // LUI (load upper immediate) is used to build 32-bit constants and uses the U-type format. LUI
    // places the 32-bit U-immediate value into the destination register rd, filling in the lowest
    // 12 bits with zeros.
    pub fn behaviorLUI(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let t1: u32 = self.imm_31_12 << 12;
        r.setReg(self.rd, t1);
    }

    // AUIPC (add upper immediate to pc) is used to build pc-relative addresses and uses the U-type
    // format. AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with
    // zeros, adds this offset to the address of the AUIPC inpub struction, then places the result
    // in register rd.
    pub fn behaviorAUIPC(&self, r: &mut register::Register, m: &mut memory::Memory) {
        let mut t1: u32 = self.imm_31_12 << 12;
        t1 += r.getPC();
        r.setReg(self.rd, t1);
    }
}

#[derive(Debug)]
pub struct UTypeDecoder {
    rd          : u32,
    imm_31_12   : u32,
}

impl UTypeDecoder {
    pub fn new(inst: u32) -> UTypeDecoder {
        let bf: UTypeBitFields = UTypeBitFields::new();
        UTypeDecoder {
            rd          : (inst & bf.rd) >> 7,
            imm_31_12   : (inst & bf.imm_31_12) >> 12 ,
        }
    }
}

// J-Type

#[derive(Debug)]
pub struct JTypeBitFields {
    rd          : u32,
    imm_19_12   : u32,
    imm_11      : u32,
    imm_10_1    : u32,
    imm_20      : u32,
}

impl JTypeBitFields {
    pub fn new() -> JTypeBitFields {
        JTypeBitFields {
            rd          : 0x0000_0F80,
            imm_19_12   : 0x0000_F000,
            imm_11      : 0x0001_0000,
            imm_10_1    : 0x07FE_0000,
            imm_20      : 0x8000_0000,
        }
    }
}

#[derive(Debug)]
pub struct JTypeDecoder {
    pub rd          : u32,
    pub imm_19_12   : u32,
    pub imm_11      : u32,
    pub imm_10_1    : u32,
    pub imm_20      : u32,
}

impl JTypeDecoder {
    pub fn new(inst: u32) -> JTypeDecoder {
        let bf: JTypeBitFields = JTypeBitFields::new();
        JTypeDecoder {
            rd          : (inst & bf.rd) >> 7,
            imm_19_12   : (inst & bf.imm_19_12) >> 12 ,
            imm_11      : (inst & bf.imm_11) >> 20 ,
            imm_10_1    : (inst & bf.imm_10_1) >> 21,
            imm_20      : (inst & bf.imm_20) >> 31,
        }
    }

    // The jump and link (JAL) instruction uses the J-type format, where the J-immediate encodes a
    // signed offset in multiples of 2 bytes. The offset is sign-extended and added to the address
    // of the jump instruction to form the jump target address. Jumps can therefore target a ±1 MiB
    // range. JAL stores the address of the instruction following the jump (pc+4) into register rd.
    // The standard software calling convention uses x1 as the return address register and x5 as an
    // alternate link register.
    pub fn behaviorJAL(&self, r: &mut register::Register, m: &mut memory::Memory) {
        r.setReg(self.rd, r.getPC + 4);
        let t1: u32 = (self.imm_20 << 19
            | self.imm_19_12 << 11
            | self.imm_11 << 10
            | self.imm_10_1);
        let t2: u32 = bittools::signExtend(t1, 20);
        // todo: u32で計算 -> 常に前方アドレスに飛ぶ前提
        r.setPC(r.getPC + t2);
    }

    pub fn behavior(&self, r: &mut register::Register, m: &mut memory::Memory) {

    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::*;

    #[test]
    fn testJTypeDecoder() {
        let inst: u32 = 0x7434_8A7E;
        let mut jtype_dec: JTypeDecoder = JTypeDecoder::new(inst);
        // assert_eq!();
    }
}

