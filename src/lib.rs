#![no_std]
extern crate alloc;

/// Defines a trait that allows for the execution of arbitrary instructions
pub mod execute_instruction;
/// Defines the format of an instruction
pub mod instruction;
/// Defines the arguments that can be used for instructions.
pub mod instruction_arguments;
pub mod syscall_handler;
/// Structs for defining the format for a vxl binary.
pub mod vxl_file;
