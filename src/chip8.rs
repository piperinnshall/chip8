use bit_vec::BitVec;
use std::ops::RangeInclusive;

const DISPLAY_SIZE: usize = (crate::WIDTH * crate::HEIGHT) as usize;
const MEMORY_SIZE: usize = 0x1000;
const FONT_RANGE: RangeInclusive<usize> = 0x050..=0x09F;

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: BitVec,
    keypad: BitVec,
    pc: usize,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
}

impl Chip8 {
    pub fn key_down(&mut self, scancode: usize) {
        self.keypad.set(scancode, true);
        println!("{}: {}", scancode, self.keypad[scancode])
    }

    pub fn key_up(&mut self, scancode: usize) {
        self.keypad.set(scancode, false);
        println!("{}: {}", scancode, self.keypad[scancode])
    }

    pub fn update(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

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
        let display = BitVec::from_fn(DISPLAY_SIZE, |i| {
            let x = (i % crate::WIDTH as usize) as i16;
            let y = (i / crate::WIDTH as usize) as i16;
            (x + y) % 2 == 0
        });
        let mut memory = [0; MEMORY_SIZE];
        memory[FONT_RANGE].copy_from_slice(&[
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
            // display: BitVec::from_elem(DISPLAY_SIZE, false),
            keypad: BitVec::from_elem(16, false),
            pc: 0x200,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
        }
    }
}
