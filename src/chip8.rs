use bit_vec::BitVec;

const MEMORY_SIZE: usize = 0x1000;
const MEMORY_START: usize = 0x200;

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: BitVec,
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    delay: u8,
    sound: u8,
    v: [u8; 16],
}

impl Chip8 {
    pub fn update(&mut self) {}
    pub fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .zip(self.display.iter())
            .for_each(|(pixel, bit)| {
                pixel.copy_from_slice(&if bit {
                    [0x5e, 0x48, 0xe8, 0xff]
                } else {
                    [0x48, 0xb2, 0xe8, 0xff]
                })
            });
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let display = BitVec::from_fn((crate::WIDTH * crate::HEIGHT) as usize, |i| {
            let x = (i % crate::WIDTH as usize) as i16;
            let y = (i / crate::WIDTH as usize) as i16;
            (x + y) % 2 == 0
        });
        let mut memory = [0; MEMORY_SIZE];
        memory[0x050..=0x09F].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
        Self {
            memory,
            display,
            pc: 0,
            i: 0,
            stack: Vec::new(),
            delay: 0,
            sound: 0,
            v: [0; 16],
        }
    }
}
