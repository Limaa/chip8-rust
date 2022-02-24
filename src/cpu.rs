use crate::ram::Ram;
use crate::instruction::{Instruction, InstructionError};

pub struct Cpu {
    registers: [u8; 16],
    i: u8,
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

    #[test]
    fn test_execute_clear_display() {
        todo!();
    }

    #[test]
    fn test_execute_return() {
        let mut cpu = Cpu::new();
        cpu.execute(Instruction::Return);
        todo!();
    }

    #[test]
    fn test_execute_jump() {
        let mut cpu = Cpu::new();
        cpu.execute(Instruction::Jump(0x238));
        assert_eq!(cpu.pc, 0x238);
    }

    #[test]
    fn test_execute_call() {
        todo!();
    }

    #[test]
    fn test_execute_skip_if_equals_byte() {
        todo!();
    }

    #[test]
    fn test_execute_skip_if_not_equals_byte() {
        todo!();
    }

    #[test]
    fn test_execute_skip_if_equals_register() {
        todo!();
    }

    #[test]
    fn test_execute_load_byte() {
        todo!();
    }

    #[test]
    fn test_execute_add_byte() {
        todo!();
    }

    #[test]
    fn test_execute_move() {
        todo!();
    }

    #[test]
    fn test_execute_or() {
        todo!();
    }

    #[test]
    fn test_execute_and() {
        todo!();
    }

    #[test]
    fn test_execute_xor() {
        todo!();
    }

    #[test]
    fn test_execute_add() {
        todo!();
    }

    #[test]
    fn test_execute_subtract() {
        todo!();
    }

    #[test]
    fn test_execute_shift_right() {
        todo!();
    }

    #[test]
    fn test_execute_subtract_reverse() {
        todo!();
    }

    #[test]
    fn test_execute_shif_left() {
        todo!();
    }

    #[test]
    fn test_execute_skip_if_not_equals_register() {
        todo!();
    }

    #[test]
    fn test_execute_load_index() {
        todo!();
    }

    #[test]
    fn test_execute_jump_with_offset() {
        todo!();
    }

    #[test]
    fn test_execute_random_with_mask() {
        todo!();
    }

    #[test]
    fn test_execute_draw() {
        todo!();
    }

    #[test]
    fn test_execute_skip_if_pressed() {
        todo!();
    }

    #[test]
    fn test_execute_skip_if_not_pressed() {
        todo!();
    }

    #[test]
    fn test_execute_load_delay_timer() {
        todo!();
    }

    #[test]
    fn test_execute_wait_key_press() {
        todo!();
    }

    #[test]
    fn test_execute_store_delay_timer() {
        todo!();
    }

    #[test]
    fn test_execute_store_sound_timer() {
        todo!();
    }

    #[test]
    fn test_execute_add_to_index() {
        todo!();
    }

    #[test]
    fn test_execute_load_sprite() {
        todo!();
    }

    #[test]
    fn test_execute_store_bcd() {
        todo!();
    }

    #[test]
    fn test_execute_store_registers() {
        todo!();
    }

    #[test]
    fn test_execute_load_registers() {
        todo!();
    }
}