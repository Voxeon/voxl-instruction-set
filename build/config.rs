use crate::constants::{ADDRESS_TYPE_NAME, IMMEDIATE_TYPE_NAME, REGISTER_TYPE_NAME};
use serde::Deserialize;

pub type InstructionDetails = Vec<Instruction>;

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Row {
    pub binary: String,
    pub decimal: u8,
    pub hex: String,
    pub opcode: String,
    pub description: String,
    pub immediates: u8,
    pub registers: u8,
    pub addresses: u8,
    pub order: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub short_name: String, // opcode
    pub fields: Vec<String>,
    pub name: String,
    pub opcode_num: u8,
    pub argument_order: Vec<char>,
}

impl Instruction {
    pub fn new(
        short_name: String,
        fields: Vec<&str>,
        name: String,
        opcode_num: u8,
        argument_order: &str,
    ) -> Self {
        return Self {
            short_name,
            fields: fields.into_iter().map(|s| s.to_string()).collect(),
            name,
            opcode_num,
            argument_order: argument_order.chars().collect(),
        };
    }
}

impl From<Row> for Instruction {
    fn from(row: Row) -> Self {
        let mut opcode = row.opcode.clone();
        let variant_name = format!("{}{}", opcode.remove(0).to_uppercase().to_string(), opcode);
        let mut fields = Vec::new();

        for _ in 0..row.immediates {
            fields.push(IMMEDIATE_TYPE_NAME);
        }

        for _ in 0..row.registers {
            fields.push(REGISTER_TYPE_NAME);
        }

        for _ in 0..row.addresses {
            fields.push(ADDRESS_TYPE_NAME);
        }

        return Self::new(row.opcode, fields, variant_name, row.decimal, &row.order);
    }
}
