pub mod core;
pub mod register;
pub mod memory;

use std::collections::HashMap;

trait Decode {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>);
}

#[derive(Debug)]
pub struct BitFields {
    OPCODE  : OpcodeBitField,
    RTYPE   : RTypeBitField,
    ITYPE   : ITypeBitField,
    STYPE   : STypeBitField,
    BTYPE   : BTypeBitField,
    UTYPE   : UTypeBitField,
    JTYPE   : JTypeBitField,
}

impl BitFields {
    pub fn new() -> BitFields {
        BitFields {
            OPCODE  : OpcodeBitField::new(),
            RTYPE   : RTypeBitField::new(),
            ITYPE   : ITypeBitField::new(),
            STYPE   : STypeBitField::new(),
            BTYPE   : BTypeBitField::new(),
            UTYPE   : UTypeBitField::new(),
            JTYPE   : JTypeBitField::new(),
        }
    }
}

#[derive(Debug)]
pub struct OpcodeBitField {
    opcode      : u32,
}

impl OpcodeBitField {
    pub fn new() -> OpcodeBitField {
        OpcodeBitField {
            opcode      : 0x0000_007F,
        }
    }
}

impl Decode for OpcodeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>){
        fields.insert("opcode", inst & self.opcode);
    }
}

#[derive(Debug)]
pub struct RTypeBitField {
    rd          : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    funct7      : u32,
}

impl RTypeBitField {
    pub fn new() -> RTypeBitField {
        RTypeBitField {
            rd          : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            funct7      : 0xFE00_0000,
        }
    }

    // ADD performs the addition of rs1 and rs2. SUB performs the subtraction of rs2 from rs1. Overflows
    // are ignored and the low XLEN bits of results are written to the destination rd.
    pub fn behaviorADD(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] + f["rs2"];
        r.setReg(f["rd"], t);
    }

    pub fn behaviorSUB(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] - f["rs2"];
        r.setReg(f["rd"], t);
    }
    
    // SLT and SLTU perform signed and unsigned compares respectively, writing 1 to rd if rs1 < rs2, 0 otherwise. Note,
    // SLTU rd, x0, rs2 sets rd to 1 if rs2 is not equal to zero, otherwise sets rd to zero (assembler
    // pseudoinstruction SNEZ rd, rs).
    pub fn behaviorSLT(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        if f["rs1"] < f["rs2"] {
            r.setReg(f["rd"], 1);
        } else {
            r.setReg(f["rd"], 0);
        }
    }
    
    pub fn behaviorSLTU(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        if (f["rs1"] as u32) < (f["rs2"] as u32) {
            r.setReg(f["rd"], 1);
        } else {
            r.setReg(f["rd"], 0);
        }
    }
    
    // AND, OR, and XOR perform bitwise logical operations.
    pub fn behaviorAND(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] & f["rs2"];
        r.setReg(f["rd"], t);
    }
    
    pub fn behaviorOR(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] | f["rs2"];
        r.setReg(f["rd"], t);
    }

    pub fn behaviorXOR(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] ^ f["rs2"];
        r.setReg(f["rd"], t);
    }

    // SLL, SRL, and SRA perform logical left, logical right, and arithmetic right shifts on the value in
    // register rs1 by the shift amount held in the lower 5 bits of register rs2.
    pub fn behaviorSLL(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] << f["rs2"];
        r.setReg(f["rd"], t);
    }
        
    pub fn behaviorSRL(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["rs1"] >> f["rs2"];
        r.setReg(f["rd"], t);
    }
    
    pub fn behaviorSRA(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let sign_bit: u32 = f["rs1"] >> 4;
        let start_pos: u32 = if f["rs2"] >= 5 { 0 } else { 4 - f["rs2"] };
        let end_pos: u32 = 32;
        let mut vacated_upper_bits: u32 = 0;
        for i in start_pos..end_pos {
            vacated_upper_bits |= sign_bit << i;
        }

        let t = vacated_upper_bits | f["rs1"] >> f["rs2"];
        r.setReg(f["rd"], t);
    }

    pub fn behavior(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        
    }
}

impl Decode for RTypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("rd", inst & self.rd);
        fields.insert("funct3", inst & self.funct3);
        fields.insert("rs1", inst & self.rs1);
        fields.insert("rs2", inst & self.rs2);
        fields.insert("funct7", inst & self.funct7);
    }
}

#[derive(Debug)]
pub struct ITypeBitField {
    rd          : u32,
    funct3      : u32,
    rs1         : u32,
    imm_11_0    : u32,
    imm_4_0     : u32,
    imm_11_5    : u32,
}

impl ITypeBitField {
    pub fn new() -> ITypeBitField {
        ITypeBitField {
            rd          : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            imm_11_0    : 0xFFF0_0000,
            imm_4_0     : 0x01F0_0000,
            imm_11_5    : 0xFE00_0000,
        }
    }

    // ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and
    // the result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement the MV
    // rd, rs1 assembler pseudoinpub struction.
    pub fn behaviorADDI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["imm_11_0"] + r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    // SLTI (set less than immediate) places the value 1 in register rd if register rs1 is less than the signextended
    // immediate when both are treated as signed numbers, else 0 is written to rd. SLTIU is
    // similar but compares the values as unsigned numbers (i.e., the immediate is first sign-extended to
    // XLEN bits then treated as an unsigned number). Note, SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals
    // zero, otherwise sets rd to 0 (assembler pseudoinpub struction SEQZ rd, rs).
    pub fn behaviorSLTI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        if f["rs1"] < f["imm_11_0"] {
            r.setReg(f["rd"], 1);
        } else {
            r.setReg(f["rd"], 0);
        }
    }

    pub fn behaviorSLTIU(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        // todo: unsigned intに直したい
        if (f["rs1"] as i32) < (f["imm_11_0"] as i32) {
            r.setReg(f["rd"], 1);
        } else {
            r.setReg(f["rd"], 0);
        }
    }

    // ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1
    // and the sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1 performs
    // a bitwise logical inversion of register rs1 (assembler pseudoinpub struction NOT rd, rs).
    pub fn behaviorXORI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_11_0"] ^ r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    pub fn behaviorORI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_11_0"] | r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    pub fn behaviorANDI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_11_0"] & r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }
    
    // Shifts by a constant are encoded as a specialization of the I-type format. The operand to be shifted
    // is in rs1, and the shift amount is encoded in the lower 5 bits of the I-immediate field. The right
    // shift type is encoded in bit 30. SLLI is a logical left shift (zeros are shifted into the lower bits);
    // SRLI is a logical right shift (zeros are shifted into the upper bits); and SRAI is an arithmetic right
    // shift (the original sign bit is copied into the vacated upper bits).
    pub fn behaviorSLLI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["rs1"] << f["imm_4_0"];
        r.setReg(f["rd"], t);
    }
    
    pub fn behaviorSRLI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["rs1"] >> f["imm_4_0"];
        r.setReg(f["rd"], t);
    }

    // todo: 動作検証が必須
    pub fn behaviorSRAI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let sign_bit: u32 = f["rs1"] >> 4;
        let start_pos: u32 = if f["imm_4_0"] >= 5 { 0 } else { 4 - f["imm_4_0"] };
        let end_pos: u32 = 32;
        let mut vacated_upper_bits: u32 = 0;
        for i in start_pos..end_pos {
            vacated_upper_bits |= sign_bit << i;
        }

        let t: u32 = vacated_upper_bits | f["rs1"] >> f["imm_4_0"];
        r.setReg(f["rd"], t);
    }

    pub fn behavior(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        
    }
}

impl Decode for ITypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("rd", inst & self.rd);
        fields.insert("funct3", inst & self.funct3);
        fields.insert("rs1", inst & self.rs1);
        fields.insert("imm_11_0", inst & self.imm_11_0);
        fields.insert("imm_4_0", inst & self.imm_4_0);
        fields.insert("imm_11_5", inst & self.imm_11_5);
    }
}

#[derive(Debug)]
pub struct STypeBitField {
    imm_4_0     : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    imm_11_5    : u32,
}

impl STypeBitField {
    pub fn new() -> STypeBitField {
        STypeBitField {
            imm_4_0     : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            imm_11_5    : 0xFE00_0000,
        }
    }
}

impl Decode for STypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("imm_4_0", inst & self.imm_4_0);
        fields.insert("funct3", inst & self.funct3);
        fields.insert("rs1", inst & self.rs1);
        fields.insert("rs2", inst & self.rs2);
        fields.insert("imm_11_5", inst & self.imm_11_5);
    }
}

#[derive(Debug)]
pub struct BTypeBitField {
    imm_11      : u32,
    imm_4_1     : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    imm_10_5    : u32,
    imm_12      : u32,
}

impl BTypeBitField {
    pub fn new() -> BTypeBitField {
        BTypeBitField {
            imm_11      : 0x0000_0080,
            imm_4_1     : 0x0000_0F00,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            imm_10_5    : 0x7C00_0000,
            imm_12      : 0x8000_0000,
        }
    }
}

impl Decode for BTypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("imm_11", inst & self.imm_11);
        fields.insert("imm_4_1", inst & self.imm_4_1);
        fields.insert("funct3", inst & self.funct3);
        fields.insert("rs1", inst & self.rs1);
        fields.insert("rs2", inst & self.rs2);
        fields.insert("imm_10_5", inst & self.imm_10_5);
        fields.insert("imm_12", inst & self.imm_12);
    }
}

#[derive(Debug)]
pub struct UTypeBitField {
    rd          : u32,
    imm_31_12   : u32,
}

impl UTypeBitField {
    pub fn new() -> UTypeBitField {
        UTypeBitField {
            rd          : 0x0000_0F80,
            imm_31_12   : 0xFFFF_F000,
        }
    }

    // LUI (load upper immediate) is used to build 32-bit constants and uses the U-type format. LUI
    // places the 32-bit U-immediate value into the destination register rd, filling in the lowest 12 bits
    // with zeros.
    pub fn behaviorLUI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_31_12"] << 12;
        r.setReg(f["rd"], t);
    }

    // AUIPC (add upper immediate to pc) is used to build pc-relative addresses and uses the U-type
    // format. AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros,
    // adds this offset to the address of the AUIPC inpub struction, then places the result in register rd.
    pub fn behaviorAUIPC(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let mut t: u32 = f["imm_31_12"] << 12;
        t += r.getPC();
        r.setReg(f["rd"], t);
    }
}

impl Decode for UTypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("rd", self.rd);
        fields.insert("imm_31_12", self.imm_31_12);
    }
}

#[derive(Debug)]
pub struct JTypeBitField {
    rd          : u32,
    imm_19_12   : u32,
    imm_11      : u32,
    imm_10_1    : u32,
    imm_20      : u32,
}

impl JTypeBitField {
    pub fn new() -> JTypeBitField {
        JTypeBitField {
            rd          : 0x0000_0F80,
            imm_19_12   : 0x0000_F000,
            imm_11      : 0x0001_0000,
            imm_10_1    : 0x07FE_0000,
            imm_20      : 0x8000_0000,
        }
    }
}

impl Decode for JTypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("rd", inst & self.rd);
        fields.insert("imm_19_12", inst & self.imm_19_12);
        fields.insert("imm_11", inst & self.imm_11);
        fields.insert("imm_10_1", inst & self.imm_10_1);
        fields.insert("imm_20", inst & self.imm_20);
    }
}

#[derive(Debug)]
pub struct CPU {
    pc: usize,
    reg: [u32; 32],
    ram: [u32; 256],
    rom: [u32; 256],
}

impl CPU {
    // fn readOpcode(&self, inst: u32, op_bf: OpBitField) -> u32 {
    //     let opcode = inst & op_bf.opcode;
    //     return opcode;
    // }

    // fn readOtherFields(&self, inst: u32, xtype_bf: BitFields) -> HashMap {
    //     let mut fields: HashMap = HashMap::new();
    //     for 
    //     return fields;
    // }

    /*
    fn read_funct3(self, inst: u32) -> u8 {
        let funct3 = ((inst & 0x0000_7000) >> 12) as u8;
        return funct3;
    }

    fn read_funct7(self, inst: u32) -> u8 {
        let funct7 = ((inst & 0xFE00_0000) >> 25) as u8;
        return funct7;
    }
    */

    fn run(&mut self) {
        let bf = BitFields::new();
        let mut reg = register::Register::new();
        let mut mem = memory::Memory::new();

        loop {
            let inst: u32 = self.rom[self.pc] as u32;
            let mut fields: HashMap<&str, u32> = HashMap::new();

            bf.OPCODE.readFields(inst, &mut fields);

            // OpcodeからTypeを特定して他フィールドを読み出し
            match fields["opcode"] {
                Opcode::LOAD        => ,
                Opcode::LOAD_FP     => ,
                Opcode::MISC_MEM    => ,
                Opcode::OP_IMM      => {
                    bf.ITYPE.readFields(inst, &mut fields);
                    match f["funct3"] {
                        Funct3OpImm::ADDI       => bf.ITYPE.behaviorADDI(fields, &mut reg, &mut mem);
                        Funct3OpImm::SLTI       => bf.ITYPE.behaviorSLTI(fields, &mut reg, &mut mem);
                        Funct3OpImm::SLLI       => bf.ITYPE.behaviorSLLI(fields, &mut reg, &mut mem);
                        Funct3OpImm::SLTIU      => bf.ITYPE.behaviorSLTIU(fields, &mut reg, &mut mem);
                        Funct3OpImm::XORI       => bf.ITYPE.behaviorXORI(fields, &mut reg, &mut mem);
                        Funct3OpImm::SRLISRAI   => {
                            match f["imm_11_5"] {
                                0b000_0000      => bf.ITYPE.behaviorSRLI(fields, &mut reg, &mut mem);
                                0b000_0001      => bf.ITYPE.behaviorSRAI(fields, &mut reg, &mut mem);
                                _               => ,
                            }
                        }
                        Funct3OpImm::ORI        => bf.ITYPE.behaviorORI(fields, &mut reg, &mut mem);
                        Funct3OpImm::ANDI       => bf.ITYPE.behaviorANDI(fields, &mut reg, &mut mem);
                        _                       => ,
                    }
                },
                Opcode::AUIPC       => {
                    bf.UTYPE.readFields(inst, &mut fields);
                    bf.UTYPE.behaviorAUIPC(fields, &mut reg, &mut mem);
                },
                Opcode::OP_IMM_32   => ,
                Opcode::STORE       => ,
                Opcode::STORE_FP    => ,
                Opcode::AMO         => ,
                Opcode::OP          => {
                    bf.RTYPE.readFields(inst, &mut fields)
                    match f["funct3"] {
                        Funct3Op::ADDSUB        => {
                            match f["funct7"] {
                                0b000_0000      => // ADD
                                0b010_0000      => // SUB
                                _               => ,
                            }
                        }
                        Funct3Op::SLT           => 
                        Funct3Op::SLTU          => 
                        Funct3Op::XOR           => 
                        Funct3Op::SRLSRA        => {
                            match f["funct7"] {
                                0b000_0000      => // SRL
                                0b010_0000      => // SRA
                                _               => ,
                            }
                        }
                        Funct3Op::OR            => 
                        Funct3Op::AND           => 
                        _                       => ,
                    }
                },
                Opcode::LUI         => ,
                    bf.UTYPE.readFields(inst, &mut fields);
                    bf.UTYPE.behaviorLUI(fields, r: &mut reg, &mut mem);
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
                _                   => ,
            }
            
            // memo: いつでもPCを4インクリメント？
            reg.incPC();

        }
    }
}

fn main() {
    let bf = BitFields::new();  
    println!("{:b}", bf.OPCODE.opcode);

    let inst: u32 = 0x8485_4744;
    println!("inst: {:x}", inst);

    let mut fields: HashMap<&str, u32> = HashMap::new();
    bf.OPCODE.readFields(inst, &mut fields);
    println!("op: {:x}", fields["opcode"]);
}

