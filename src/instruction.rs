use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum RegisterName {
    Zero,
    AT,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    GP,
    SP,
    FP,
    RA,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Rformat = 0b000000,
    Addi = 0b001000,
    Addiu = 0b001001,
    Andi = 0b001100,
    Beq = 0b000100,
    Bne = 0b000101,
    Jump = 0b000010,
    Jal = 0b000011,
    Lbu = 0b100100,
    Lhu = 0b100101,
    Lui = 0b001111,
    Lw = 0b100011,
    Ori = 0b001101,
    Slti = 0b001010,
    Sltiu = 0b001011,
    Sb = 0b101000,
    Sh = 0b101001,
    Sw = 0b101011,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Funct {
    Add = 0b100000,
    Addu = 0b100001,
    And = 0b100100,
    Jr = 0b001000,
    Nor = 0b100111,
    Or = 0b100101,
    Slt = 0b101010,
    Sltu = 0b101011,
    Sll = 0b000000,
    Srl = 0b000010,
    Sub = 0b100010,
    Subu = 0b100011,
    Div = 0b011010,
    Divu = 0b011011,
    Mfhi = 0b010000,
    Mflo = 0b010010,
    Mult = 0b011000,
    Multu = 0b011001,
    Sra = 0b000011,
    Syscall = 0b001100,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RFormat {
    label: Option<String>,
    rs: RegisterName,
    rt: RegisterName,
    rd: RegisterName,
    shamt: u8,
    funct: Funct,
}

#[allow(dead_code)]
impl RFormat {
    pub fn new(
        funct: Funct,
        rs: RegisterName,
        rt: RegisterName,
        rd: RegisterName,
        shamt: u8,
    ) -> RFormat {
        RFormat {
            label: None,
            rs,
            rt,
            rd,
            shamt,
            funct,
        }
    }
    pub fn lebel_new(
        label: String,
        funct: Funct,
        rs: RegisterName,
        rt: RegisterName,
        rd: RegisterName,
        shamt: u8,
    ) -> RFormat {
        RFormat {
            label: Some(label),
            rs,
            rt,
            rd,
            shamt,
            funct,
        }
    }
}

impl Instruction for RFormat {
    fn convert(&self) -> u32 {
        let mut instruction: u32;
        instruction = (self.rs as u32 & 0b11111) << 21;
        instruction |= (self.rt as u32 & 0b11111) << 16;
        instruction |= (self.rd as u32 & 0b11111) << 11;
        instruction |= (self.shamt as u32 & 0b11111) << 6;
        instruction |= self.funct as u32 & 0b111111;
        instruction
    }

    fn label_to_address(&mut self, _current_address: u32, _table: &HashMap<String, u32>) {}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct IFormat {
    label: Option<String>,
    opcode: OpCode,
    rs: RegisterName,
    rt: RegisterName,
    immediate: ImmediateOrLabel,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ImmediateOrLabel {
    Immediate(i16),
    Label(String),
}

#[allow(dead_code)]
impl IFormat {
    pub fn new(opcode: OpCode, rs: RegisterName, rt: RegisterName, immediate: i16) -> IFormat {
        IFormat {
            label: None,
            opcode,
            rs,
            rt,
            immediate: ImmediateOrLabel::Immediate(immediate),
        }
    }

    pub fn new_label(opcode: OpCode, rs: RegisterName, rt: RegisterName, label: String) -> IFormat {
        IFormat {
            label: None,
            opcode,
            rs,
            rt,
            immediate: ImmediateOrLabel::Label(label),
        }
    }

    pub fn label_new(
        label: String,
        opcode: OpCode,
        rs: RegisterName,
        rt: RegisterName,
        immediate: i16,
    ) -> IFormat {
        IFormat {
            label: Some(label),
            opcode,
            rs,
            rt,
            immediate: ImmediateOrLabel::Immediate(immediate),
        }
    }

    pub fn label_new_label(
        label: String,
        opcode: OpCode,
        rs: RegisterName,
        rt: RegisterName,
        target_label: String,
    ) -> IFormat {
        IFormat {
            label: Some(label),
            opcode,
            rs,
            rt,
            immediate: ImmediateOrLabel::Label(target_label),
        }
    }
}

#[allow(dead_code)]
impl Instruction for IFormat {
    fn convert(&self) -> u32 {
        match self.immediate {
            ImmediateOrLabel::Immediate(immediate) => {
                let mut instruction: u32;
                instruction = (self.opcode as u32 & 0b111111) << 26;
                instruction |= (self.rs as u32 & 0b11111) << 21;
                instruction |= (self.rt as u32 & 0b11111) << 16;
                instruction |= immediate as u16 as u32;
                instruction
            }
            _ => panic!("label must be replaced to address"),
        }
    }

    fn label_to_address(&mut self, current_address: u32, table: &HashMap<String, u32>) {
        match self.opcode {
            OpCode::Beq | OpCode::Bne => {
                if let ImmediateOrLabel::Label(label) = &self.immediate {
                    let target_address = *table.get(label).expect("invalid label") as i16;
                    let relative_address = target_address - current_address as i16 - 1;
                    self.immediate = ImmediateOrLabel::Immediate(relative_address);
                }
            }
            _ => (),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct JFormat {
    label: Option<String>,
    opcode: OpCode,
    address: AddressOrLabel,
}

#[allow(dead_code)]
#[derive(Debug)]
enum AddressOrLabel {
    Address(u32),
    Label(String),
}

#[allow(dead_code)]
impl JFormat {
    pub fn new(opcode: OpCode, address: u32) -> JFormat {
        JFormat {
            label: None,
            opcode,
            address: AddressOrLabel::Address(address),
        }
    }

    pub fn new_label(opcode: OpCode, label: String) -> JFormat {
        JFormat {
            label: None,
            opcode,
            address: AddressOrLabel::Label(label),
        }
    }

    pub fn label_new(label: String, opcode: OpCode, address: u32) -> JFormat {
        JFormat {
            label: Some(label),
            opcode,
            address: AddressOrLabel::Address(address),
        }
    }

    pub fn label_new_label(label: String, opcode: OpCode, target_label: String) -> JFormat {
        JFormat {
            label: Some(label),
            opcode,
            address: AddressOrLabel::Label(target_label),
        }
    }
}

impl Instruction for JFormat {
    fn convert(&self) -> u32 {
        match self.address {
            AddressOrLabel::Address(address) => {
                let mut instruction: u32;
                instruction = (self.opcode as u32 & 0b111111) << 26;
                instruction |= address & 0x03FFFFFF;
                instruction
            }
            _ => panic!("label must be replaced to address"),
        }
    }

    fn label_to_address(&mut self, _current_address: u32, table: &HashMap<String, u32>) {
        match self.opcode {
            OpCode::Jump | OpCode::Jal => {
                if let AddressOrLabel::Label(label) = &self.address {
                    let address = *table.get(label).expect("invalid label");
                    self.address = AddressOrLabel::Address(address);
                }
            }
            _ => panic!("invalid instruction: {:?}", self),
        }
    }
}

pub trait Instruction: std::fmt::Debug {
    fn convert(&self) -> u32;
    fn label_to_address(&mut self, current_address: u32, table: &HashMap<String, u32>);
}
