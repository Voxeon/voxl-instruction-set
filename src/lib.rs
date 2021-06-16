#![no_std]
extern crate alloc;

mod execute_instruction;
mod instruction;
mod instruction_arguments;

pub use execute_instruction::ExecuteInstruction;
pub use instruction::Instruction;
pub use instruction_arguments::{Address, Immediate, InstructionArgument, Register};
