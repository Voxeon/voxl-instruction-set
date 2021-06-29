use crate::open_file_writing;
use crate::constants::*;
use std::io::Write;
use crate::config::*;

pub fn generate_execute_trait(instructions: &InstructionDetails) {
    let mut file = open_file_writing(EXECUTE_OUTPUT_PATH);

    file.write(execute_header().as_bytes())
        .expect("Unable to write execute trait header");

    for instruction in instructions {
        let mut field_names = Vec::new();

        let mut reg_count = 0;
        let mut imm_count = 0;
        let mut add_count = 0;

        for field in &instruction.fields {
            if field == REGISTER_TYPE_NAME {
                field_names.push(format!(
                    "r{}",
                    if reg_count == 0 {
                        String::new()
                    } else {
                        reg_count.to_string()
                    }
                ));

                reg_count += 1;
            } else if field == IMMEDIATE_TYPE_NAME {
                field_names.push(format!(
                    "i{}",
                    if imm_count == 0 {
                        String::new()
                    } else {
                        imm_count.to_string()
                    }
                ));

                imm_count += 1;
            } else if field == ADDRESS_TYPE_NAME {
                field_names.push(format!(
                    "a{}",
                    if add_count == 0 {
                        String::new()
                    } else {
                        add_count.to_string()
                    }
                ));

                add_count += 1;
            }
        }

        let field_string = field_names.join(", ");

        if field_string.len() > 0 {
            file.write(
                format!(
                    "\t\t\t{}::{}({}) => self.execute_{}({}),\n",
                    OPCODE_ENUM_NAME, &instruction.name, &field_string, &instruction.short_name, field_string,
                )
                    .as_bytes(),
            )
                .expect("Failed to write match arm for execute function");
        } else {
            file.write(
                format!(
                    "\t\t\t{}::{} => self.execute_{}(),\n",
                    OPCODE_ENUM_NAME, &instruction.name, &instruction.short_name
                )
                    .as_bytes(),
            )
                .expect("Failed to write match arm for execute function");
        }
    }

    file.write("\t\t};\n\t}\n\n".as_bytes())
        .expect("Unable to write execute_opcode function tail");

    for instruction in instructions {
        let mut field_names = Vec::new();

        let mut reg_count = 0;
        let mut imm_count = 0;
        let mut add_count = 0;

        for field in &instruction.fields {
            if field == REGISTER_TYPE_NAME {
                field_names.push(format!(
                    "r{}: {}",
                    if reg_count == 0 {
                        String::new()
                    } else {
                        reg_count.to_string()
                    },
                    REGISTER_TYPE_NAME
                ));

                reg_count += 1;
            } else if field == IMMEDIATE_TYPE_NAME {
                field_names.push(format!(
                    "i{}: {}",
                    if imm_count == 0 {
                        String::new()
                    } else {
                        imm_count.to_string()
                    },
                    IMMEDIATE_TYPE_NAME
                ));

                imm_count += 1;
            } else if field == "Address" {
                field_names.push(format!(
                    "a{}: {}",
                    if add_count == 0 {
                        String::new()
                    } else {
                        add_count.to_string()
                    },
                    ADDRESS_TYPE_NAME
                ));

                add_count += 1;
            }
        }

        let mut field_string = field_names.join(", ");

        if field_string.len() > 0 {
            field_string = format!(", {}", field_string);
        }

        file.write(
            format!(
                "\tfn execute_{}(&mut self{}) -> Self::Output;\n\n",
                &instruction.short_name, &field_string
            )
                .as_bytes(),
        )
            .expect("Failed to write execute opcode function");
    }

    file.write("}\n\n".as_bytes())
        .expect("Unable to write execute_opcode function tail");
}