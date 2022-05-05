pub mod core;
pub mod register;
pub mod memory;

use std::collections::HashMap;

trait Decode {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>);
}

#[derive(Debug)]
struct BitFields {
    OPCODE  : OpcodeBitField,
    RTYPE   : RTypeBitField,
    ITYPE   : ITypeBitField,
    STYPE   : STypeBitField,
    BTYPE   : BTypeBitField,
    UTYPE   : UTypeBitField,
    JTYPE   : JTypeBitField,
}

impl BitFields {
    fn new() -> BitFields {
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
struct OpcodeBitField {
    opcode      : u32,
}

impl OpcodeBitField {
    fn new() -> OpcodeBitField {
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
struct RTypeBitField {
    rd          : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    funct7      : u32,
}

impl RTypeBitField {
    fn new() -> RTypeBitField {
        RTypeBitField {
            rd          : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            rs2         : 0x01F0_0000,
            funct7      : 0xFE00_0000,
        }
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
struct ITypeBitField {
    rd          : u32,
    funct3      : u32,
    rs1         : u32,
    imm_11_0    : u32,
}

impl ITypeBitField {
    fn new() -> ITypeBitField {
        ITypeBitField {
            rd          : 0x0000_0F80,
            funct3      : 0x0000_7000,
            rs1         : 0x000F_8000,
            imm_11_0    : 0xFFF0_0000,
        }
    }

    // ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and
    // the result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement the MV
    // rd, rs1 assembler pseudoinstruction.
    fn behaviorADDI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t = f["imm_11_0"] + r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    // SLTI (set less than immediate) places the value 1 in register rd if register rs1 is less than the signextended
    // immediate when both are treated as signed numbers, else 0 is written to rd. SLTIU is
    // similar but compares the values as unsigned numbers (i.e., the immediate is first sign-extended to
    // XLEN bits then treated as an unsigned number). Note, SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals
    // zero, otherwise sets rd to 0 (assembler pseudoinstruction SEQZ rd, rs).
    fn behaviorSLTI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        if f["rs1"] < f["imm_11_0"] {
            r.setReg(f["rd"], 1);
        } else {
            r.setReg(f["rd"], 0);
        }
    }

    fn behaviorSLTIU(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        // todo: unsigned intに直したい
        if (f["rs1"] as i32) < (f["imm_11_0"] as i32) {
            r.setReg(f["rd"], 1);
        } else {
            r.setReg(f["rd"], 0);
        }
    }

    // ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1
    // and the sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1 performs
    // a bitwise logical inversion of register rs1 (assembler pseudoinstruction NOT rd, rs).
    fn behaviorXORI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_11_0"] ^ r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    fn behaviorORI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_11_0"] | r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    fn behaviorANDI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_11_0"] & r.getReg(f["rs1"]);
        r.setReg(f["rd"], t);
    }

    fn behavior(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        
    }
}

impl Decode for ITypeBitField {
    fn readFields(&self, inst: u32, fields: &mut HashMap<&str, u32>) {
        fields.insert("rd", inst & self.rd);
        fields.insert("funct3", inst & self.funct3);
        fields.insert("rs1", inst & self.rs1);
        fields.insert("imm_11_0", inst & self.imm_11_0);
    }
}

#[derive(Debug)]
struct STypeBitField {
    imm_4_0     : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    imm_11_5    : u32,
}

impl STypeBitField {
    fn new() -> STypeBitField {
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
struct BTypeBitField {
    imm_11      : u32,
    imm_4_1     : u32,
    funct3      : u32,
    rs1         : u32,
    rs2         : u32,
    imm_10_5    : u32,
    imm_12      : u32,
}

impl BTypeBitField {
    fn new() -> BTypeBitField {
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
struct UTypeBitField {
    rd          : u32,
    imm_31_12   : u32,
}

impl UTypeBitField {
    fn new() -> UTypeBitField {
        UTypeBitField {
            rd          : 0x0000_0F80,
            imm_31_12   : 0xFFFF_F000,
        }
    }

    // LUI (load upper immediate) is used to build 32-bit constants and uses the U-type format. LUI
    // places the 32-bit U-immediate value into the destination register rd, filling in the lowest 12 bits
    // with zeros.
    fn behaviorLUI(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_31_12"] << 12;
        r.setReg(f["rd"], t);
    }

    // AUIPC (add upper immediate to pc) is used to build pc-relative addresses and uses the U-type
    // format. AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros,
    // adds this offset to the address of the AUIPC instruction, then places the result in register rd.
    fn behaviorAUIPC(&self, f: HashMap<&str, u32>, r: &mut register::Register, m: &mut memory::Memory) {
        let t: u32 = f["imm_31_12"] << 12;
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
struct JTypeBitField {
    rd          : u32,
    imm_19_12   : u32,
    imm_11      : u32,
    imm_10_1    : u32,
    imm_20      : u32,
}

impl JTypeBitField {
    fn new() -> JTypeBitField {
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
struct CPU {
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
            let mut fields: HashMap = HashMap::new();

            bf.OPCODE.readFields(inst, &mut fields);

            // OpcodeからTypeを特定して他フィールドを読み出し
            match fields.get("opcode") {
                Opcode::LOAD        => ,
                Opcode::LOAD_FP     => ,
                Opcode::MISC_MEM    => ,
                Opcode::OP_IMM      => {
                    bf.RTYPE.readFields(inst, &mut fields);
                    match fields.get("funct3") {
                        Funct3OpImm::ADDI       => ,
                        Funct3OpImm::SLTI       => ,
                        Funct3OpImm::SLTIU      => ,
                        Funct3OpImm::XORI       => ,
                        Funct3OpImm::ORI        => ,
                        Funct3OpImm::ANDI       => ,
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
                Opcode::OP          => ,
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

