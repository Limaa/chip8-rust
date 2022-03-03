use crate::ram::Ram;
use crate::instruction::{Instruction, InstructionError};

pub struct Cpu {
    registers: [u8; 16],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    pc: u16,
    sp: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    fn fetch(&self, memory: Ram) -> u16 {
        memory.word(self.pc)
    }

    fn decode(instruction: u16) -> Result<Instruction, InstructionError> {
        Instruction::new(instruction)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearDisplay => {unimplemented!()}
            Instruction::Return => {unimplemented!()}
            Instruction::Jump(addr) => {self.pc = addr;},
            Instruction::Call(addr) => {unimplemented!()}
            Instruction::SkipIfEqualsByte(x, num) => {unimplemented!()}
            Instruction::SkipIfNotEqualsByte(x, num) => {unimplemented!()}
            Instruction::SkipIfEqualsRegister(x, y) => {unimplemented!()}
            Instruction::LoadByte(x, num) => {unimplemented!()}
            Instruction::AddByte(x, num) => {unimplemented!()}
            Instruction::Move(x, num) => {unimplemented!()}
            Instruction::Or(x, y) => {unimplemented!()}
            Instruction::And(x, y) => {unimplemented!()}
            Instruction::Xor(x, y) => {unimplemented!()}
            Instruction::Add(x, y) => {unimplemented!()}
            Instruction::Subtract(x, y) => {unimplemented!()}
            Instruction::ShiftRight(x, y) => {unimplemented!()}
            Instruction::SubtractReverse(x, y) => {unimplemented!()}
            Instruction::ShifLeft(x, y) => {unimplemented!()}
            Instruction::SkipIfNotEqualsRegister(x, y) => {unimplemented!()}
            Instruction::LoadIndex(addr) => {unimplemented!()}
            Instruction::JumpWithOffset(addr) => {unimplemented!()}
            Instruction::RandomWithMask(x, mask) => {unimplemented!()}
            Instruction::Draw(x, y, n) => {unimplemented!()}
            Instruction::SkipIfPressed(x) => {unimplemented!()}
            Instruction::SkipIfNotPressed(x) => {unimplemented!()}
            Instruction::LoadDelayTimer(x) => {unimplemented!()}
            Instruction::WaitKeyPress(x) => {unimplemented!()}
            Instruction::StoreDelayTimer(x) => {unimplemented!()}
            Instruction::StoreSoundTimer(x) => {unimplemented!()}
            Instruction::AddToIndex(x) => {unimplemented!()}
            Instruction::LoadSprite(x) => {unimplemented!()}
            Instruction::StoreBCD(x) => {unimplemented!()}
            Instruction::StoreRegisters(x) => {unimplemented!()}
            Instruction::LoadRegisters(x) => {unimplemented!()}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use super::Instruction;

    // #[test]
    // fn test_execute_clear_display() {
    //     todo!();
    // }

    #[test]
    fn test_execute_return() {
        let mut cpu = Cpu::new();
        cpu.sp = 1;
        cpu.stack[0] = 0x381;
        cpu.pc = 0x245;
        cpu.execute(Instruction::Return);

        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 0x381);
    }

    #[test]
    fn test_execute_jump() {
        let mut cpu = Cpu::new();
        cpu.execute(Instruction::Jump(0x238));
        assert_eq!(cpu.pc, 0x238);
    }

    #[test]
    fn test_execute_call() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.execute(Instruction::Call(0x821));

        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[0], 0x247);
        assert_eq!(cpu.pc, 0x821);
    }

    #[test]
    fn test_execute_skip_if_equals_byte_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 38;
        cpu.execute(Instruction::SkipIfEqualsByte(2, 38));
        assert_eq!(cpu.pc, 0x249);
    }

    #[test]
    fn test_execute_skip_if_equals_byte_not_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 29;
        cpu.execute(Instruction::SkipIfEqualsByte(2, 38));
        assert_eq!(cpu.pc, 0x248);
    }

    #[test]
    fn test_execute_skip_if_not_equals_byte_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 29;
        cpu.execute(Instruction::SkipIfNotEqualsByte(2, 38));
        assert_eq!(cpu.pc, 0x249);
    }


    #[test]
    fn test_execute_skip_if_not_equals_byte_not_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 38;
        cpu.execute(Instruction::SkipIfNotEqualsByte(2, 38));
        assert_eq!(cpu.pc, 0x248);
    }

    #[test]
    fn test_execute_skip_if_equals_register_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 38;
        cpu.registers[3] = 38;
        cpu.execute(Instruction::SkipIfEqualsRegister(2, 3));
        assert_eq!(cpu.pc, 0x249);
    }

    #[test]
    fn test_execute_skip_if_equals_register_not_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 29;
        cpu.registers[3] = 83;
        cpu.execute(Instruction::SkipIfEqualsRegister(2, 3));
        assert_eq!(cpu.pc, 0x248);
    }

    #[test]
    fn test_execute_load_byte() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 38;
        cpu.execute(Instruction::LoadByte(2, 3));
        assert_eq!(cpu.pc, 0x248);
        assert_eq!(cpu.registers[2], 3);
    }

    #[test]
    fn test_execute_add_byte() {
        let mut cpu = Cpu::new();
        cpu.registers[6] = 39;
        cpu.execute(Instruction::AddByte(6, 3));
        assert_eq!(cpu.registers[6], 39 + 3);
    }

    #[test]
    fn test_execute_move() {
        let mut cpu = Cpu::new();
        cpu.registers[6] = 39;
        cpu.registers[8] = 42;
        cpu.execute(Instruction::Move(6, 8));
        assert_eq!(cpu.registers[6], 42);
    }

    #[test]
    fn test_execute_or() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 39;
        cpu.registers[7] = 42;
        cpu.execute(Instruction::Or(2, 7));
        assert_eq!(cpu.registers[2], 39 | 42);
    }

    #[test]
    fn test_execute_and() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 39;
        cpu.registers[7] = 42;
        cpu.execute(Instruction::And(2, 7));
        assert_eq!(cpu.registers[2], 39 & 42);
    }

    #[test]
    fn test_execute_xor() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 39;
        cpu.registers[7] = 42;
        cpu.execute(Instruction::Xor(2, 7));
        assert_eq!(cpu.registers[2], 39 ^ 42);
    }

    #[test]
    fn test_execute_add() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 39;
        cpu.registers[7] = 42;
        cpu.execute(Instruction::Add(2, 7));
        assert_eq!(cpu.registers[2], 39 + 42);
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn test_execute_add_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 200;
        cpu.registers[7] = 100;
        cpu.execute(Instruction::Add(2, 7));
        assert_eq!(cpu.registers[2], 200u8.wrapping_add(100));
        assert_eq!(cpu.registers[0xF], 1);
    }

    #[test]
    fn test_execute_subtract() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 200;
        cpu.registers[7] = 100;
        cpu.execute(Instruction::Subtract(2, 7));
        assert_eq!(cpu.registers[2], 200 - 100);
        assert_eq!(cpu.registers[0xF], 1);
    }

    #[test]
    fn test_execute_subtract_with_borrow() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 100;
        cpu.registers[7] = 200;
        cpu.execute(Instruction::Subtract(2, 7));
        assert_eq!(cpu.registers[2], 100u8.wrapping_sub(200));
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn test_execute_shift_right_with_carry() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 85;
        cpu.execute(Instruction::ShiftRight(2, 7));
        assert_eq!(cpu.registers[2], 85>>1);
        assert_eq!(cpu.registers[0xF], 85&1);
    }

    #[test]
    fn test_execute_shift_right_no_carry() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 84;
        cpu.execute(Instruction::ShiftRight(2, 7));
        assert_eq!(cpu.registers[2], 84>>1);
        assert_eq!(cpu.registers[0xF], 84&1);
    }

    #[test]
    fn test_execute_subtract_reverse_with_borrow() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 200;
        cpu.registers[7] = 100;
        cpu.execute(Instruction::SubtractReverse(2, 7));
        assert_eq!(cpu.registers[2], 100u8.wrapping_sub(200));
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn test_execute_subtract_reverse() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 100;
        cpu.registers[7] = 200;
        cpu.execute(Instruction::SubtractReverse(2, 7));
        assert_eq!(cpu.registers[2], 200 - 100);
        assert_eq!(cpu.registers[0xF], 1);
    }

    #[test]
    fn test_execute_shif_left_with_carry() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 149;
        cpu.execute(Instruction::ShifLeft(2, 7));
        assert_eq!(cpu.registers[2], 149<<1);
        assert_eq!(cpu.registers[0xF], 149&0x80);
    }

    #[test]
    fn test_execute_shif_left_no_carry() {
        let mut cpu = Cpu::new();
        cpu.registers[2] = 21;
        cpu.execute(Instruction::ShifLeft(2, 7));
        assert_eq!(cpu.registers[2], 21<<1);
        assert_eq!(cpu.registers[0xF], 21&0x80);
    }

    #[test]
    fn test_execute_skip_if_not_equals_register_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 38;
        cpu.registers[3] = 39;
        cpu.execute(Instruction::SkipIfNotEqualsRegister(2, 3));
        assert_eq!(cpu.pc, 0x249);
    }

    #[test]
    fn test_execute_skip_if_not_equals_register_not_skipping() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x247;
        cpu.registers[2] = 42;
        cpu.registers[3] = 42;
        cpu.execute(Instruction::SkipIfNotEqualsRegister(2, 3));
        assert_eq!(cpu.pc, 0x248);
    }

    #[test]
    fn test_execute_load_index() {
        let mut cpu = Cpu::new();
        cpu.i = 0x292;
        cpu.execute(Instruction::LoadIndex(0x182));
        assert_eq!(cpu.i, 0x182);
    }

    #[test]
    fn test_execute_jump_with_offset() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x492;
        cpu.registers[0] = 0x2;
        cpu.execute(Instruction::JumpWithOffset(0x132));
        assert_eq!(cpu.pc, 0x132 + 0x2);
    }

    // #[test]
    // fn test_execute_random_with_mask() {
    //     todo!();
    // }

    // #[test]
    // fn test_execute_draw() {
    //     todo!();
    // }

    // #[test]
    // fn test_execute_skip_if_pressed() {
    //     todo!();
    // }

    // #[test]
    // fn test_execute_skip_if_not_pressed() {
    //     todo!();
    // }

    #[test]
    fn test_execute_load_delay_timer() {
        let mut cpu = Cpu::new();
        cpu.registers[0xA] = 0x2;
        cpu.delay_timer = 0x5;
        cpu.execute(Instruction::LoadDelayTimer(0xA));
        assert_eq!(cpu.registers[0xA], 0x5);
        assert_eq!(cpu.delay_timer, 0x5);
    }

    // #[test]
    // fn test_execute_wait_key_press() {
    //     todo!();
    // }

    #[test]
    fn test_execute_store_delay_timer() {
        let mut cpu = Cpu::new();
        cpu.registers[0xA] = 0x2;
        cpu.delay_timer = 0x5;
        cpu.execute(Instruction::StoreDelayTimer(0xA));
        assert_eq!(cpu.registers[0xA], 0x2);
        assert_eq!(cpu.delay_timer, 0x2);
    }

    #[test]
    fn test_execute_store_sound_timer() {
        let mut cpu = Cpu::new();
        cpu.registers[0xA] = 0x2;
        cpu.sound_timer = 0x5;
        cpu.execute(Instruction::StoreSoundTimer(0xA));
        assert_eq!(cpu.registers[0xA], 0x2);
        assert_eq!(cpu.sound_timer, 0x2);
    }

    #[test]
    fn test_execute_add_to_index() {
        let mut cpu = Cpu::new();
        cpu.i = 0x4;
        cpu.registers[0xA] = 0x2;
        cpu.execute(Instruction::AddToIndex(0xA));
        assert_eq!(cpu.i, 0x4 + 0x2);
    }

    // #[test]
    // fn test_execute_load_sprite() {
    //     todo!();
    // }

    // #[test]
    // fn test_execute_store_bcd() {
    //     todo!();
    // }

    // #[test]
    // fn test_execute_store_registers() {
    //     todo!();
    // }

    // #[test]
    // fn test_execute_load_registers() {
    //     todo!();
    // }
}
