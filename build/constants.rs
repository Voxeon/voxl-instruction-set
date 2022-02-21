pub const BASE_IMPORTS: [&'static str; 5] = [
    "use alloc::vec::Vec",
    "use alloc::vec",
    "use alloc::format",
    "use alloc::string::String",
    "use alloc::string::ToString",
];
pub const INSTRUCTION_SET_PATH: &'static str = "base_instruction_set.csv";
pub const OPCODE_OUTPUT_PATH: &'static str = "src/instruction.rs";
pub const EXECUTE_OUTPUT_PATH: &'static str = "src/execute_instruction.rs";
pub const OPCODE_ENUM_NAME: &'static str = "Instruction";
pub const REGISTER_TYPE_NAME: &'static str = "Register";
pub const ADDRESS_TYPE_NAME: &'static str = "Address";
pub const IMMEDIATE_TYPE_NAME: &'static str = "Immediate";
pub const INSTRUCTION_ARGUMENT_TRAIT_NAME: &'static str = "InstructionArgument";
pub const EXECUTE_TRAIT_NAME: &'static str = "ExecuteInstruction";
pub const OPCODE_DERIVE_TRAITS: &'static str = "#[derive(Debug, Clone, Copy, PartialEq)]";
pub const OPCODE_REGISTER_COUNT_METHOD_HEADER: &'static str = "register_count";
pub const OPCODE_ADDRESS_COUNT_METHOD_HEADER: &'static str = "address_count";
pub const OPCODE_IMMEDIATE_COUNT_METHOD_HEADER: &'static str = "immediate_count";

pub const NEW_OPCODE_OPENING_IF:[&'static str; 3] =
    ["if registers.len() != Self::register_count(opcode)? || addresses.len() != Self::address_count(opcode)? || immediates.len() != Self::immediate_count(opcode)? {",
        "\treturn None;", "}"];
