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
    pub fn label_new(
        label: &str,
        funct: Funct,
        rs: RegisterName,
        rt: RegisterName,
        rd: RegisterName,
        shamt: u8,
    ) -> RFormat {
        RFormat {
            label: Some(label.to_string()),
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

    fn to_string(&self) -> String {
        let mut str = String::new();

        if let Some(label) = &self.label {
            str.push_str(label);
            str.push(':');
        }
        str.push('\t');
        match self.funct {
            Funct::Add => str.push_str("add "),
            Funct::Addu => str.push_str("addu "),
            Funct::And => str.push_str("and "),
            Funct::Jr => str.push_str("jr "),
            Funct::Nor => str.push_str("nor "),
            Funct::Or => str.push_str("or "),
            Funct::Slt => str.push_str("slt "),
            Funct::Sltu => str.push_str("sltu "),
            Funct::Sll => str.push_str("sll "),
            Funct::Srl => str.push_str("srl "),
            Funct::Sub => str.push_str("sub "),
            Funct::Subu => str.push_str("subu "),
            Funct::Div => str.push_str("div "),
            Funct::Divu => str.push_str("divu "),
            Funct::Mfhi => str.push_str("mfhi "),
            Funct::Mflo => str.push_str("mflo "),
            Funct::Mult => str.push_str("mult "),
            Funct::Multu => str.push_str("multu "),
            Funct::Sra => str.push_str("sra "),
            Funct::Syscall => str.push_str("syscall "),
        }
        match self.funct {
            Funct::Add
            | Funct::Addu
            | Funct::And
            | Funct::Jr
            | Funct::Nor
            | Funct::Or
            | Funct::Slt
            | Funct::Sltu
            | Funct::Sll
            | Funct::Srl
            | Funct::Sub
            | Funct::Subu
            | Funct::Div
            | Funct::Divu
            | Funct::Mult
            | Funct::Multu => {
                str.push_str(&self.rs.to_string());
                str.push(' ');
            }
            _ => (),
        }
        match self.funct {
            Funct::Add
            | Funct::Addu
            | Funct::And
            | Funct::Nor
            | Funct::Or
            | Funct::Slt
            | Funct::Sltu
            | Funct::Sub
            | Funct::Subu
            | Funct::Div
            | Funct::Divu
            | Funct::Mult
            | Funct::Multu
            | Funct::Sra => {
                str.push_str(&self.rt.to_string());
                str.push(' ');
            }
            _ => (),
        }
        match self.funct {
            Funct::Add
            | Funct::Addu
            | Funct::And
            | Funct::Nor
            | Funct::Or
            | Funct::Slt
            | Funct::Sltu
            | Funct::Sll
            | Funct::Srl
            | Funct::Sub
            | Funct::Subu
            | Funct::Mfhi
            | Funct::Mflo
            | Funct::Sra => {
                str.push_str(&self.rd.to_string());
                str.push(' ');
            }
            _ => (),
        }
        match self.funct {
            Funct::Sll | Funct::Srl | Funct::Sra => str.push_str(&self.shamt.to_string()),
            _ => (),
        }

        str
    }
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
        label: &str,
        opcode: OpCode,
        rs: RegisterName,
        rt: RegisterName,
        immediate: i16,
    ) -> IFormat {
        IFormat {
            label: Some(label.to_string()),
            opcode,
            rs,
            rt,
            immediate: ImmediateOrLabel::Immediate(immediate),
        }
    }

    pub fn label_new_label(
        label: &str,
        opcode: OpCode,
        rs: RegisterName,
        rt: RegisterName,
        target_label: String,
    ) -> IFormat {
        IFormat {
            label: Some(label.to_string()),
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

    fn to_string(&self) -> String {
        let mut str = String::new();

        if let Some(label) = &self.label {
            str.push_str(label);
            str.push(':');
        }
        str.push('\t');
        match self.opcode {
            OpCode::Addi => str.push_str("addi "),
            OpCode::Addiu => str.push_str("addiu "),
            OpCode::Andi => str.push_str("andi "),
            OpCode::Beq => str.push_str("beq "),
            OpCode::Bne => str.push_str("bne "),
            OpCode::Lbu => str.push_str("lbu "),
            OpCode::Lhu => str.push_str("lhu "),
            OpCode::Lui => str.push_str("lui "),
            OpCode::Lw => str.push_str("lw "),
            OpCode::Ori => str.push_str("ori "),
            OpCode::Slti => str.push_str("slti "),
            OpCode::Sltiu => str.push_str("sltiu "),
            OpCode::Sb => str.push_str("sb "),
            OpCode::Sh => str.push_str("sh "),
            OpCode::Sw => str.push_str("sw "),
            _ => panic!("no way"),
        }
        match self.opcode {
            OpCode::Addi
            | OpCode::Addiu
            | OpCode::Andi
            | OpCode::Beq
            | OpCode::Bne
            | OpCode::Ori
            | OpCode::Slti
            | OpCode::Sltiu => {
                str.push_str(&self.rs.to_string());
                str.push(' ');
            }
            _ => (),
        }
        match self.opcode {
            OpCode::Addi
            | OpCode::Addiu
            | OpCode::Andi
            | OpCode::Beq
            | OpCode::Bne
            | OpCode::Lbu
            | OpCode::Lhu
            | OpCode::Lui
            | OpCode::Lw
            | OpCode::Ori
            | OpCode::Slti
            | OpCode::Sltiu
            | OpCode::Sb
            | OpCode::Sh
            | OpCode::Sw => {
                str.push_str(&self.rt.to_string());
                str.push(' ');
            }
            _ => (),
        }
        match self.opcode {
            OpCode::Lbu | OpCode::Lhu | OpCode::Lw | OpCode::Sb | OpCode::Sh | OpCode::Sw => {
                if let ImmediateOrLabel::Immediate(immediate) = self.immediate {
                    str.push_str(&immediate.to_string());
                }
                str.push('(');
                str.push_str(&self.rs.to_string());
                str.push(')');
            }
            _ => match &self.immediate {
                ImmediateOrLabel::Immediate(immediate) => {
                    str.push_str(&immediate.to_string());
                }
                ImmediateOrLabel::Label(label) => {
                    str.push_str(label);
                }
            },
        }

        str
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
pub enum AddressOrLabel {
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

    pub fn label_new(label: &str, opcode: OpCode, address: u32) -> JFormat {
        JFormat {
            label: Some(label.to_string()),
            opcode,
            address: AddressOrLabel::Address(address),
        }
    }

    pub fn label_new_label(label: &str, opcode: OpCode, target_label: String) -> JFormat {
        JFormat {
            label: Some(label.to_string()),
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

    fn to_string(&self) -> String {
        let mut str = String::new();

        if let Some(label) = &self.label {
            str.push_str(label);
            str.push(':');
        }
        str.push('\t');
        match self.opcode {
            OpCode::Jump => str.push_str("j "),
            OpCode::Jal => str.push_str("jal "),
            _ => panic!("no way"),
        }
        match &self.address {
            AddressOrLabel::Address(address) => {
                str.push_str(&address.to_string());
            }
            AddressOrLabel::Label(label) => {
                str.push_str(label);
            }
        }

        str
    }
}

pub trait Instruction: std::fmt::Debug {
    fn convert(&self) -> u32;
    fn label_to_address(&mut self, current_address: u32, table: &HashMap<String, u32>);
    fn to_string(&self) -> String;
}

impl RegisterName {
    fn to_string(&self) -> String {
        match self {
            RegisterName::Zero => String::from("$zero"),
            RegisterName::AT => String::from("$at"),
            RegisterName::V0 => String::from("$v0"),
            RegisterName::V1 => String::from("$v1"),
            RegisterName::A0 => String::from("$a0"),
            RegisterName::A1 => String::from("$a1"),
            RegisterName::A2 => String::from("$a2"),
            RegisterName::A3 => String::from("$a3"),
            RegisterName::T0 => String::from("$t0"),
            RegisterName::T1 => String::from("$t1"),
            RegisterName::T2 => String::from("$t2"),
            RegisterName::T3 => String::from("$t3"),
            RegisterName::T4 => String::from("$t4"),
            RegisterName::T5 => String::from("$t5"),
            RegisterName::T6 => String::from("$t6"),
            RegisterName::T7 => String::from("$t7"),
            RegisterName::S0 => String::from("$s0"),
            RegisterName::S1 => String::from("$s1"),
            RegisterName::S2 => String::from("$s2"),
            RegisterName::S3 => String::from("$s3"),
            RegisterName::S4 => String::from("$s4"),
            RegisterName::S5 => String::from("$s5"),
            RegisterName::S6 => String::from("$s6"),
            RegisterName::S7 => String::from("$s7"),
            RegisterName::T8 => String::from("$t8"),
            RegisterName::T9 => String::from("$t9"),
            RegisterName::K0 => String::from("$k0"),
            RegisterName::K1 => String::from("$k1"),
            RegisterName::GP => String::from("$gp"),
            RegisterName::SP => String::from("$sp"),
            RegisterName::FP => String::from("$fp"),
            RegisterName::RA => String::from("$ra"),
        }
    }
}
