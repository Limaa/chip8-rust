type Addr = u16;
type Reg = u8;
type Byte = u8;
type Nibble = u8;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    // SYS,
    ClearDisplay,
    Return,
    Jump(Addr),
    Call(Addr),
    SkipIfEqualsByte(Reg, Byte),
    SkipIfNotEqualsByte(Reg, Byte),
    SkipIfEqualsRegister(Reg, Reg),
    LoadByte(Reg, Byte),
    AddByte(Reg, Byte),
    Move(Reg, Reg),
    Or(Reg, Reg),
    And(Reg, Reg),
    Xor(Reg, Reg),
    Add(Reg, Reg),
    Subtract(Reg, Reg),
    ShiftRight(Reg, Reg),
    SubtractReverse(Reg, Reg),
    ShifLeft(Reg, Reg),
    SkipIfNotEqualsRegister(Reg, Reg),
    LoadIndex(Addr),
    JumpWithOffset(Addr),
    RandomWithMask(Reg, Byte),
    Draw(Reg, Reg, Nibble),
    SkipIfPressed(Reg),
    SkipIfNotPressed(Reg),
    LoadDelayTimer(Reg),
    WaitKeyPress(Reg),
    StoreDelayTimer(Reg),
    StoreSoundTimer(Reg),
    AddToIndex(Reg),
    LoadSprite(Reg),
    StoreBCD(Reg),
    StoreRegisters(Reg),
    LoadRegisters(Reg),
}

#[derive(Debug)]
pub enum InstructionError {
    InvalidInstruction,
}

impl Instruction {
    pub fn new(instruction: u16) -> Result<Instruction, InstructionError> {
        let x = ((0x0F00 & instruction) >> 8) as u8;
        let y = ((0x00F0 & instruction) >> 4) as u8;
        let n = ((0x000F & instruction) >> 0) as u8;
        let kk = ((0x00FF & instruction) >> 0) as u8;
        let nnn = ((0x0FFF & instruction) >> 0) as u16;

        match instruction {
            0x00E0 => Ok(Instruction::ClearDisplay),
            0x00EE => Ok(Instruction::Return),
            0x1000..=0x1FFF => Ok(Instruction::Jump(nnn)),
            0x2000..=0x2FFF => Ok(Instruction::Call(nnn)),
            0x3000..=0x3FFF => Ok(Instruction::SkipIfEqualsByte(x, kk)),
            0x4000..=0x4FFF => Ok(Instruction::SkipIfNotEqualsByte(x, kk)),
            0x5000..=0x5FFF => Ok(Instruction::SkipIfEqualsRegister(x, y)),
            0x6000..=0x6FFF => Ok(Instruction::LoadByte(x, kk)),
            0x7000..=0x7FFF => Ok(Instruction::AddByte(x, kk)),
            0x8000..=0x8FFF => {
                match n {
                    0x0 => Ok(Instruction::Move(x, y)),
                    0x1 => Ok(Instruction::Or(x, y)),
                    0x2 => Ok(Instruction::And(x, y)),
                    0x3 => Ok(Instruction::Xor(x, y)),
                    0x4 => Ok(Instruction::Add(x, y)),
                    0x5 => Ok(Instruction::Subtract(x, y)),
                    0x6 => Ok(Instruction::ShiftRight(x, y)),
                    0x7 => Ok(Instruction::SubtractReverse(x, y)),
                    0xE => Ok(Instruction::ShifLeft(x, y)),
                    _ => Err(InstructionError::InvalidInstruction)
                }
            },
            0x9000..=0x9FFF => Ok(Instruction::SkipIfNotEqualsRegister(x, y)),
            0xA000..=0xAFFF => Ok(Instruction::LoadIndex(nnn)),
            0xB000..=0xBFFF => Ok(Instruction::JumpWithOffset(nnn)),
            0xC000..=0xCFFF => Ok(Instruction::RandomWithMask(x, kk)),
            0xD000..=0xDFFF => Ok(Instruction::Draw(x, y, n)),
            0xE000..=0xEFFF => {
                match kk {
                    0x9E => Ok(Instruction::SkipIfPressed(x)),
                    0xA1 => Ok(Instruction::SkipIfNotPressed(x)),
                    _ => Err(InstructionError::InvalidInstruction)
                }
            },
            0xF000..=0xFFFF => {
                match kk {
                    0x07 => Ok(Instruction::LoadDelayTimer(x)),
                    0x0A => Ok(Instruction::WaitKeyPress(x)),
                    0x15 => Ok(Instruction::StoreDelayTimer(x)),
                    0x18 => Ok(Instruction::StoreSoundTimer(x)),
                    0x1E => Ok(Instruction::AddToIndex(x)),
                    0x29 => Ok(Instruction::LoadSprite(x)),
                    0x33 => Ok(Instruction::StoreBCD(x)),
                    0x55 => Ok(Instruction::StoreRegisters(x)),
                    0x65 => Ok(Instruction::LoadRegisters(x)),
                    _ => Err(InstructionError::InvalidInstruction)
                }
            },
            _ => Err(InstructionError::InvalidInstruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn test_decode_clear_display() {
        let instruction = Instruction::new(0x00E0).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::ClearDisplay);
    }

    #[test]
    fn test_decode_return() {
        let instruction = Instruction::new(0x00EE).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Return);
    }

    #[test]
    fn test_decode_jump() {
        let instruction = Instruction::new(0x1A2A).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Jump(0x0A2A));
    }

    #[test]
    fn test_decode_call() {
        let instruction = Instruction::new(0x2F4A).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Call(0x0F4A));
    }

    #[test]
    fn test_decode_skip_if_equals_byte() {
        let instruction = Instruction::new(0x3B72).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SkipIfEqualsByte(0xB, 0x72));
    }

    #[test]
    fn test_decode_skip_if_not_equals_byte() {
        let instruction = Instruction::new(0x4B72).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SkipIfNotEqualsByte(0xB, 0x72));
    }

    #[test]
    fn test_decode_skip_if_equals_register() {
        let instruction = Instruction::new(0x5BC2).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SkipIfEqualsRegister(0xB, 0xC));
    }

    #[test]
    fn test_decode_load_byte() {
        let instruction = Instruction::new(0x63A7).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::LoadByte(0x3, 0xA7));
    }

    #[test]
    fn test_decode_add_byte() {
        let instruction = Instruction::new(0x7543).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::AddByte(0x5, 0x43));
    }

    #[test]
    fn test_decode_move() {
        let instruction = Instruction::new(0x8BC0).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Move(0xB, 0xC));
    }

    #[test]
    fn test_decode_or() {
        let instruction = Instruction::new(0x8371).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Or(0x3, 0x7));
    }

    #[test]
    fn test_decode_and() {
        let instruction = Instruction::new(0x8422).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::And(0x4, 0x2));
    }

    #[test]
    fn test_decode_xor() {
        let instruction = Instruction::new(0x8753).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Xor(0x7, 0x5));
    }

    #[test]
    fn test_decode_add() {
        let instruction = Instruction::new(0x8C34).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Add(0xC, 0x3));
    }

    #[test]
    fn test_decode_subtract() {
        let instruction = Instruction::new(0x83B5).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Subtract(0x3, 0xB));
    }

    #[test]
    fn test_decode_shift_right() {
        let instruction = Instruction::new(0x82A6).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::ShiftRight(0x2, 0xA));
    }

    #[test]
    fn test_decode_subtract_reverse() {
        let instruction = Instruction::new(0x8C87).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SubtractReverse(0xC, 0x8));
    }

    #[test]
    fn test_decode_shif_left() {
        let instruction = Instruction::new(0x842E).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::ShifLeft(0x4, 0x2));
    }

    #[test]
    fn test_decode_skip_if_not_equals_register() {
        let instruction = Instruction::new(0x9C43).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SkipIfNotEqualsRegister(0xC, 0x4));
    }

    #[test]
    fn test_decode_load_index() {
        let instruction = Instruction::new(0xA527).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::LoadIndex(0x0527));
    }

    #[test]
    fn test_decode_jump_with_offset() {
        let instruction = Instruction::new(0xB82A).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::JumpWithOffset(0x082A));
    }

    #[test]
    fn test_decode_random_with_mask() {
        let instruction = Instruction::new(0xC82A).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::RandomWithMask(0x8, 0x2A));
    }

    #[test]
    fn test_decode_draw() {
        let instruction = Instruction::new(0xD8E1).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::Draw(0x8, 0xE, 0x1));
    }

    #[test]
    fn test_decode_skip_if_pressed() {
        let instruction = Instruction::new(0xEA9E).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SkipIfPressed(0xA));
    }

    #[test]
    fn test_decode_skip_if_not_pressed() {
        let instruction = Instruction::new(0xEBA1).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::SkipIfNotPressed(0xB));
    }

    #[test]
    fn test_decode_load_delay_timer() {
        let instruction = Instruction::new(0xF707).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::LoadDelayTimer(0x7));
    }

    #[test]
    fn test_decode_wait_key_press() {
        let instruction = Instruction::new(0xF20A).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::WaitKeyPress(0x2));
    }

    #[test]
    fn test_decode_store_delay_timer() {
        let instruction = Instruction::new(0xF415).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::StoreDelayTimer(0x4));
    }

    #[test]
    fn test_decode_store_sound_timer() {
        let instruction = Instruction::new(0xFB18).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::StoreSoundTimer(0xB));
    }

    #[test]
    fn test_decode_add_to_index() {
        let instruction = Instruction::new(0xF51E).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::AddToIndex(0x5));
    }

    #[test]
    fn test_decode_load_sprite() {
        let instruction = Instruction::new(0xF629).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::LoadSprite(0x6));
    }

    #[test]
    fn test_decode_store_bcd() {
        let instruction = Instruction::new(0xF133).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::StoreBCD(0x1));
    }

    #[test]
    fn test_decode_store_registers() {
        let instruction = Instruction::new(0xF855).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::StoreRegisters(0x8));
    }

    #[test]
    fn test_decode_load_registers() {
        let instruction = Instruction::new(0xF965).expect("Error decoding instruction");
        assert_eq!(instruction, Instruction::LoadRegisters(0x9));
    }
}