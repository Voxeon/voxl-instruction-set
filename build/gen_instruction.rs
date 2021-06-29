use crate::config::*;
use std::fs::{File, OpenOptions};
use crate::constants::*;
use std::io::Write;

macro_rules! create_property_method {
    ($header:expr, $method_name:ident, $type_name:ident) => {
        fn $method_name(base_indent: &[u8], file: &mut File, instructions: &InstructionDetails) {
            file.write(base_indent)
                .expect("Unable to write base indent");
            file.write($header.as_bytes())
                .expect("Unable to write property method header.");
            file.write(base_indent)
                .expect("Unable to write base indent");
            file.write(b"\t return Some(match opcode {\n")
                .expect("Unable to write step indent");

            for instruction in instructions {
                let dec = instruction.opcode_num;

                file.write(base_indent)
                    .expect("Unable to write base indent");
                file.write(b"\t\t").expect("Unable to write step indent");

                let mut count = 0;

                for field in &instruction.fields {
                    if field == $type_name {
                        count += 1;
                    }
                }

                write!(file, "{} => {}, //{}\n", dec, count, &instruction.name)
                    .expect("Unable to write opcode match line");
            }

            if instructions.len() != 256 {
                file.write(base_indent)
                    .expect("Unable to write base indent");
                file.write(b"\t\t").expect("Unable to write step indent");
                write!(file, "_ => return None,\n")
                    .expect("Unable to write field display information.");
            }

            file.write(base_indent)
                .expect("Unable to write base indent");
            file.write(b"\t});\n").expect("Unable to write next indent");
            file.write(base_indent)
                .expect("Unable to write base indent");
            file.write(b"}\n")
                .expect("Unable to write method termination");
        }
    };
}


pub fn generate_instruction_file(instruction_details: &InstructionDetails) {
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(OPCODE_OUTPUT_PATH)
    {
        Ok(f) => f,
        Err(e) => {
            panic!(
                "Failed to open the file {}. Error: {}",
                INSTRUCTION_SET_PATH, e
            );
        }
    };

    file.write_all(opcode_header().as_bytes())
        .expect("Unable to write header to file.");

    for instruction in instruction_details {
        let mut enum_variant = format!("\t{}", &instruction.name);

        if instruction.fields.len() != 0 {
            enum_variant.push('(');
            enum_variant.push_str(&instruction.fields.join(", "));
            enum_variant.push(')');
        }

        enum_variant.push_str(", \n");

        file.write(enum_variant.as_bytes())
            .expect("Unable to write a row");
    }

    file.write(OPCODE_TAIL.as_bytes())
        .expect("Unable to write tail to file.");

    generate_opcode_methods(&mut file, &instruction_details);

    file.write(opcode_impl_display_header().as_bytes())
        .expect("Unable to write header to file.");

    // Generate the display trait
    for instruction in instruction_details {
        let mut lhs = format!("\t\t\tSelf::{}", &instruction.name);

        if instruction.fields.len() > 0 {
            lhs.push('(');

            for _ in 0..instruction.fields.len() - 1 {
                lhs.push_str("_, ");
            }

            lhs.push_str("_)");
        }

        write!(file, "{} => \"{}\",\n", lhs, &instruction.short_name)
            .expect("Unable to write field display information.");
    }

    file.write_all("\t\t});\n\t}\n}\n\n".as_bytes())
        .expect("Unable to write display tail to file.");

    generate_into_bytes_trait(&mut file, &instruction_details);

    file.write_all(b"\t\t};\n\t}\n}")
        .expect("Unable to write into trait tail to file");
}

fn generate_opcode_methods(file: &mut File, opcodes: &InstructionDetails) {
    file.write(opcode_impl_header().as_bytes())
        .expect("Failed to write opcode impl header.");

    generate_ordering_array(opcodes, file, "\t");

    generate_register_method(b"\t", file, opcodes);

    file.write(b"\n").expect("Failed to write padding new line");

    generate_address_method(b"\t", file, opcodes);

    file.write(b"\n").expect("Failed to write padding new line");

    generate_immediate_method(b"\t", file, opcodes);

    file.write(b"\n").expect("Failed to write padding new line");

    generate_opcode_from_name_method(b"\t", file, opcodes);

    // Generate the create new opcode
    file.write(b"\n\t")
        .expect("Failed to write padding new line");

    file.write(opcode_new_header().as_bytes())
        .expect("Failed to write Instruction \"new\" method.");

    generate_new_opcode_method(b"\t", file, opcodes);

    file.write(b"\n").expect("Failed to write padding new line");

    generate_type_ordering_function(file, "\t");

    file.write(b"}\n\n")
        .expect("Failed to write impl closing brace");
}

fn generate_opcode_from_name_method(base_indent: &[u8], file: &mut File, instructions: &InstructionDetails) {
    file.write(base_indent)
        .expect("Unable to write base indent");
    file.write(b"pub fn from_string(opcode: &str) -> Option<u8> {\n")
        .expect("Unable to write property method header.");
    file.write(base_indent)
        .expect("Unable to write base indent");
    file.write(b"\t return Some(match opcode {\n")
        .expect("Unable to write step indent");

    for instruction in instructions {
        file.write(base_indent)
            .expect("Unable to write base indent");
        file.write(b"\t\t").expect("Unable to write step indent");

        write!(file, "\"{}\" => {}, //{}\n", &instruction.short_name, instruction.opcode_num, &instruction.name)
            .expect("Unable to write opcode match line");
    }

    file.write(base_indent)
        .expect("Unable to write base indent");
    file.write(b"\t\t").expect("Unable to write step indent");
    write!(file, "_ => return None,\n").expect("Unable to write field display information.");

    file.write(base_indent)
        .expect("Unable to write base indent");
    file.write(b"\t});\n").expect("Unable to write next indent");
    file.write(base_indent)
        .expect("Unable to write base indent");
    file.write(b"}\n")
        .expect("Unable to write method termination");
}

create_property_method!(
    OPCODE_REGISTER_COUNT_METHOD_HEADER,
    generate_register_method,
    REGISTER_TYPE_NAME
);

create_property_method!(
    OPCODE_ADDRESS_COUNT_METHOD_HEADER,
    generate_address_method,
    ADDRESS_TYPE_NAME
);

create_property_method!(
    OPCODE_IMMEDIATE_COUNT_METHOD_HEADER,
    generate_immediate_method,
    IMMEDIATE_TYPE_NAME
);

fn generate_new_opcode_method(inset: &[u8], file: &mut File, instructions: &InstructionDetails) {
    for line in &NEW_OPCODE_OPENING_IF {
        file.write(inset).expect("Failed to write inset");
        file.write(b"\t").expect("Failed to write new line");

        file.write(*line)
            .expect("Failed to write opcode_opening_ifs");
        file.write(b"\n").expect("Failed to write new line");
    }

    file.write(b"\n").expect("Failed to write new line");

    file.write(inset).expect("Failed to write inset");
    file.write(b"\treturn Some(match opcode {\n")
        .expect("Failed to write match start");

    for instruction in instructions {
        let dec = instruction.opcode_num;
        file.write(inset).expect("Unable to write the inset");

        let mut arg_fields = String::new();
        let mut reg_i = 0;
        let mut add_i = 0;
        let mut imm_i = 0;

        for field in &instruction.fields {
            if field == REGISTER_TYPE_NAME {
                arg_fields.push_str(&format!("registers[{}], ", reg_i));

                reg_i += 1;
            } else if field == ADDRESS_TYPE_NAME {
                arg_fields.push_str(&format!("addresses[{}], ", add_i));

                add_i += 1;
            } else if field == IMMEDIATE_TYPE_NAME {
                arg_fields.push_str(&format!("immediates[{}], ", imm_i));

                imm_i += 1;
            }
        }

        if arg_fields.len() > 0 {
            // Remove ", "
            arg_fields.pop();
            arg_fields.pop();

            write!(file, "\t\t{} => Self::{}({}),\n", dec, &instruction.name, arg_fields)
                .expect("Unable to write field creation information.");
        } else {
            write!(file, "\t\t{} => Self::{},\n", dec, &instruction.name)
                .expect("Unable to write field creation information.");
        }
    }

    file.write(inset).expect("Unable to write the inset");
    file.write(b"\t\t_ => return None,\n")
        .expect("Failed to write general return");

    file.write(inset).expect("Failed to write inset");
    file.write(b"\t});\n").expect("Failed to write match end");

    file.write(inset).expect("Failed to write inset");
    file.write(b"}\n").expect("Failed to write ending");
}

fn generate_into_bytes_trait(file: &mut File, instructions: &InstructionDetails) {
    file.write(opcode_impl_into_bytes_header().as_bytes())
        .expect("Failed to write bytes header.");

    for instruction in instructions {
        let inset = "\t\t\t";

        file.write(inset.as_bytes())
            .expect("Unable to write step indent");

        let mut r_count = 0;
        let mut a_count = 0;
        let mut i_count = 0;

        for field in &instruction.fields {
            if field == REGISTER_TYPE_NAME {
                r_count += 1;
            } else if field == ADDRESS_TYPE_NAME {
                a_count += 1;
            } else if field == IMMEDIATE_TYPE_NAME {
                i_count += 1;
            }
        }

        let mut fields_string = String::new();
        let mut rhs_string = format!("{{\n{}\tlet mut v = vec![{}];\n", inset, instruction.opcode_num);

        for i in 0..i_count {
            if i == 0 {
                fields_string.push_str("i, ");
                rhs_string.push_str(&format!(
                    "{}\tv.extend_from_slice(&Into::<[u8; {}::BYTES]>::into(i));\n",
                    inset, IMMEDIATE_TYPE_NAME
                ));
            } else {
                fields_string.push_str(&format!("i{}, ", i));
                rhs_string.push_str(&format!(
                    "{}\tv.extend_from_slice(&Into::<[u8; {}::BYTES]>::into(i{}));\n",
                    inset, IMMEDIATE_TYPE_NAME, i
                ));
            }
        }

        fn register_name(i: u64) -> String {
            if i == 0 {
                return "r".to_string();
            } else {
                return format!("r{}", i);
            }
        }

        for i in 0..r_count {
            fields_string.push_str(&format!("{}, ", register_name(i)));

            if i > 0 && i % 2 == 1 {
                rhs_string.push_str(&format!(
                    "{}\tv.push(({} as u8) << 4 | ({} as u8));\n",
                    inset,
                    register_name(i - 1),
                    register_name(i)
                ));
            } else if i == r_count - 1 {
                rhs_string.push_str(&format!(
                    "{}\tv.push(({} as u8) << 4);\n",
                    inset,
                    register_name(i)
                ));
            }
        }

        for i in 0..a_count {
            if i == 0 {
                fields_string.push_str("a, ");
                rhs_string.push_str(&format!(
                    "{}\tv.extend_from_slice(&Into::<[u8; {}::BYTES]>::into(a));\n",
                    inset, ADDRESS_TYPE_NAME
                ));
            } else {
                fields_string.push_str(&format!("a{}, ", i));
                rhs_string.push_str(&format!(
                    "{}\tv.extend_from_slice(&Into::<[u8; {}::BYTES]>::into(a{}));\n",
                    inset, ADDRESS_TYPE_NAME, i
                ));
            }
        }

        if fields_string.len() > 1 {
            fields_string.pop(); // remove trailing space
            fields_string.pop(); // remove trailing comma

            fields_string = format!("({})", fields_string);
        }

        if i_count == 0 && r_count == 0 && a_count == 0 {
            rhs_string = format!("vec![{}]", instruction.opcode_num);
        } else {
            rhs_string = format!("{}{}\tv\n{}}}", rhs_string, inset, inset);
        }

        write!(
            file,
            "Self::{}{} => {},\n",
            instruction.name, fields_string, rhs_string
        )
            .expect("Unable to write opcode match line");
    }
}

fn generate_ordering_array(instructions: &InstructionDetails, file: &mut File, indent: &str) {
    file.write(
        format!(
            "{}const ORDERING_ARRAY: [&'static[u8]; {}] = [",
            indent,
            instructions.len()
        )
            .as_bytes(),
    )
        .expect("Failed to write generic template for constant array.");

    for instruction in instructions {
        assert_eq!(instruction.argument_order.len(), instruction.fields.len());

        write!(file, "&[").expect("Failed to write entry");
        for char in &instruction.argument_order {
            if *char == 'r' {
                write!(file, "0, ").expect("Failed to write entry");
            } else if *char == 'i' {
                write!(file, "1, ").expect("Failed to write entry");
            } else if *char == 'a' {
                write!(file, "2, ").expect("Failed to write entry");
            } else {
                panic!("Unknown order symbol {}", char);
            }
        }

        write!(file, "], ").expect("Failed to write entry");
    }

    write!(file, "];\n\n").expect("Failed to write closing brace");
}

fn generate_type_ordering_function(f: &mut File, indent: &str) {
    write!(
        f,
        "{}pub const fn get_type_for_index(opcode: u8, index: usize) -> Option<u8> {{\n",
        indent
    )
        .expect("Failed to write type ordering function line.");
    write!(
        f,
        "{}\tif opcode as usize >= Self::ORDERING_ARRAY.len() {{\n",
        indent
    )
        .expect("Failed to write type ordering function line.");
    write!(f, "{}\t\treturn None;\n", indent)
        .expect("Failed to write type ordering function line.");
    write!(f, "{}\t}}\n\n", indent).expect("Failed to write type ordering function line.");
    write!(
        f,
        "{}\tif index >= Self::ORDERING_ARRAY[opcode as usize].len() {{\n",
        indent
    )
        .expect("Failed to write type ordering function line.");
    write!(f, "{}\t\treturn None;\n", indent)
        .expect("Failed to write type ordering function line.");
    write!(f, "{}\t}}\n\n", indent).expect("Failed to write type ordering function line.");
    write!(
        f,
        "{}\treturn Some(Self::ORDERING_ARRAY[opcode as usize][index]);\n",
        indent
    )
        .expect("Failed to write type ordering function line.");
    write!(f, "{}}}\n", indent).expect("Failed to write type ordering function line.");
}
