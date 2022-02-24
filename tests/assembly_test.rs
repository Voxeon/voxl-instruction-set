use vxl_iset::instruction::Instruction;
use vxl_iset::instruction_arguments::{Address, Immediate, Register};

#[test]
fn copyi_to_assembly() {
    let instruction = Instruction::Copyi(
        Immediate::from(0u64),
        Immediate::from(1u64),
        Immediate::from(2u64),
        Register::R0,
        Register::R1,
    );

    assert_eq!(instruction.into_assembly(), "copyi $r0, 0u0, $r1, 0u1, 0u2");
}

#[test]
fn ldf_to_assembly() {
    let instruction = Instruction::Ldf(Immediate::from(12.22), Register::R0);

    assert_eq!(instruction.into_assembly(), "ldf $r0, 0f12.22");
}

#[test]
fn ldi_to_assembly() {
    let instruction = Instruction::Ldi(Immediate::from(-12i64), Register::R0);

    assert_eq!(instruction.into_assembly(), "ldi $r0, 0i-12");
}

#[test]
fn jmp_to_assmebly() {
    let instruction = Instruction::Jmp(Address::new(12));

    assert_eq!(instruction.into_assembly(), "jmp 0u12");
}
