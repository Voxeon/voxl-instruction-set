/*
This script is very messy and it will be hard to expand in the future but for the moment it works and will do.
*/
mod gen_execute_instruction;
mod gen_instruction;
mod constants;
mod config;

use std::fs::{File, OpenOptions};

use constants::*;
use gen_execute_instruction::generate_execute_trait;
use gen_instruction::generate_instruction_file;
use crate::config::{InstructionDetails, Row, Instruction};

fn main() {
    println!("cargo:rerun-if-changed={}", INSTRUCTION_SET_PATH);

    let instructions = load_method_details();

    generate_instruction_file(&instructions);
    generate_execute_trait(&instructions);
}

fn open_file_writing(path: &str) -> File {
    return match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            panic!(
                "Failed to open the file {}. Error: {}",
                EXECUTE_OUTPUT_PATH, e
            );
        }
    };
}

fn open_file_reading(path: &str) -> File {
    return match OpenOptions::new()
        .read(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            panic!(
                "Failed to open the file {}. Error: {}",
                EXECUTE_OUTPUT_PATH, e
            );
        }
    };
}

fn load_method_details() -> InstructionDetails {
    if !std::path::Path::new(INSTRUCTION_SET_PATH).exists() {
        panic!("No instruction set file found at {}", INSTRUCTION_SET_PATH);
    }

    let file = open_file_reading(INSTRUCTION_SET_PATH);
    let mut csv_reader = csv::Reader::from_reader(file);
    let mut rows = Vec::new();

    for record in csv_reader.records() {
        if let Ok(record) = record {
            match record.deserialize::<Row>(None) {
                Ok(r) => rows.push(r),
                Err(e) => {
                    panic!("Could not deserialize row. Error: {}", e);
                }
            }
        } else {
            panic!("Could not access row.");
        }
    }

    return rows.into_iter().map(|row| Instruction::from(row)).collect();
}