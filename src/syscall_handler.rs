use crate::execute_instruction::ExecuteInstruction;

macro_rules! define_default_system_call {
    ($name:ident) => {
        fn $name(&mut self, machine: &mut M) -> Option<u64>;
    };
}

pub trait SyscallHandler<M: ExecuteInstruction> {
    fn execute_call(&mut self, call: u64, machine: &mut M) -> Option<u64> {
        return match call {
            0 => self.exit(machine),

            1 => self.write_byte_terminal(machine),
            2 => self.write_terminal(machine),
            3 => self.read_byte_terminal(machine),
            4 => self.read_terminal(machine),

            5 => self.open_file(machine),
            6 => self.close_file(machine),
            7 => self.read_file(machine),
            8 => self.write_file(machine),
            9 => self.execute_file(machine),
            10 => self.execute_vxl_file(machine),
            11 => self.delete_file(machine),
            12 => self.move_file(machine),
            13 => self.copy_file(machine),

            14 => self.time_of_day(machine),

            256..=u64::MAX => self.execute_target_specific_call(call, machine),
            _ => return None,
        };
    }

    fn execute_target_specific_call(&mut self, call: u64, machine: &mut M) -> Option<u64>;

    define_default_system_call!(exit);

    define_default_system_call!(write_byte_terminal);
    define_default_system_call!(write_terminal);
    define_default_system_call!(read_byte_terminal);
    define_default_system_call!(read_terminal);

    define_default_system_call!(open_file);
    define_default_system_call!(close_file);
    define_default_system_call!(read_file);
    define_default_system_call!(write_file);
    define_default_system_call!(execute_file);
    define_default_system_call!(execute_vxl_file);
    define_default_system_call!(delete_file);
    define_default_system_call!(move_file);
    define_default_system_call!(copy_file);

    define_default_system_call!(time_of_day);
}
