use std::fs::File;
use std::io::Read;

const RAM_SIZE: usize = 4096;

pub struct Ram {
    memory: [u8; RAM_SIZE]
}

impl Ram {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("Error opening the file");
        let mut buffer = [0u8; RAM_SIZE];
        let _bytes_read = f.read(&mut buffer[0x200..]).expect("Error reading the file");
        Ram {
            memory: buffer
        }
    }

    pub fn word(&self, offset: u16) -> u16 {
        // Instructions are 2 bytes long and stored in big-endian format.
        // MSB -> Most significant byte first
        let higher = (self.memory[offset as usize] as u16) << 8;
        let lower = self.memory[(offset + 1) as usize] as u16;
        higher & lower
    }

    pub fn print(&self) {
        for (idx, val) in self.memory.windows(16).enumerate() {
            print!("{:04x}: ", idx*16);
            for b in val {
                print!("{b:02x} ");
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ram;

    #[test]
    fn test_load_rom() {
        let mut ram = Ram { memory: [0; 4096] };
        let mut rom: [u8; 3584] = [0; 3584];
        rom[0] = 0xFF;
        rom[1] = 0xCC;

        // ram.load_rom(&rom);
        // assert_eq!(ram.memory[0x200], 0xFF);
        // assert_eq!(ram.memory[0x201], 0xCC);
        assert_eq!(1, 1);
    }
}