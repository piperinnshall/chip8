use crate::opcode::Opcode;
use bit_vec::BitVec;
use std::ops::RangeInclusive;

pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;
const DISPLAY_SIZE: usize = WIDTH as usize * HEIGHT as usize;
const MEMORY_START: usize = 0x200;
const MEMORY_SIZE: usize = 0x1000;
const FONT_RANGE: RangeInclusive<usize> = 0x050..=0x09F;
const FONT: [u8; 80] = [
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
];

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: BitVec,
    keypad: BitVec,
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
    shift_vip: bool,
    jump_vip: bool,
}

impl Chip8 {
    pub fn load(&mut self, rom: &[u8]) {
        self.memory[MEMORY_START..MEMORY_START + rom.len()].copy_from_slice(rom);
    }

    pub fn ambiguous(&mut self, shift_vip: bool, jump_vip: bool) {
        self.shift_vip = shift_vip;
        self.jump_vip = jump_vip;
    }

    pub fn key_down(&mut self, scancode: usize) {
        self.keypad.set(scancode, true);
        println!("{}: {}", scancode, self.keypad[scancode])
    }

    pub fn key_up(&mut self, scancode: usize) {
        self.keypad.set(scancode, false);
        println!("{}: {}", scancode, self.keypad[scancode])
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        frame
            .chunks_exact_mut(4)
            .zip(self.display.iter())
            .for_each(|(pixel, bit)| {
                let col = if bit { 0xFF } else { 0x00 };
                pixel.copy_from_slice(&[col, col, col, 0xFF])
            });
    }

    pub fn fetch_decode_execute(&mut self) {
        let opcode = Opcode::decode(self.mem(self.pc), self.mem(self.pc + 1));
        self.pc += 2;
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::_00E0 => self.display.fill(false),
            Opcode::_00EE => self.pc = self.stack.pop().unwrap(),
            Opcode::_1NNN(nnn) => self.pc = nnn,
            Opcode::_2NNN(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Opcode::_3XNN(x, nn) => {
                if self.v(x) == nn {
                    self.pc += 2;
                }
            }
            Opcode::_4XNN(x, nn) => {
                if self.v(x) != nn {
                    self.pc += 2;
                }
            }
            Opcode::_5XY0(x, y) => {
                if self.v(x) == self.v(y) {
                    self.pc += 2;
                }
            }
            Opcode::_9XY0(x, y) => {
                if self.v(x) != self.v(y) {
                    self.pc += 2;
                }
            }
            Opcode::_6XNN(x, nn) => *self.v_mut(x) = nn,
            Opcode::_7XNN(x, nn) => *self.v_mut(x) = self.v_mut(x).wrapping_add(nn),
            Opcode::_8XY0(x, y) => *self.v_mut(x) = self.v(y),
            Opcode::_8XY1(x, y) => *self.v_mut(x) = self.v(x) | self.v(y),
            Opcode::_8XY2(x, y) => *self.v_mut(x) = self.v(x) & self.v(y),
            Opcode::_8XY3(x, y) => *self.v_mut(x) = self.v(x) ^ self.v(y),
            Opcode::_8XY4(x, y) => {
                let (res, carry) = self.v(x).overflowing_add(self.v(y));
                *self.v_mut(x) = res;
                *self.v_mut(0xF) = carry as u8;
            }
            Opcode::_8XY5(x, y) => {
                let (res, borrow) = self.v(x).overflowing_sub(self.v(y));
                *self.v_mut(x) = res;
                *self.v_mut(0xF) = (!borrow) as u8;
            }
            Opcode::_8XY7(x, y) => {
                let (res, borrow) = self.v(y).overflowing_sub(self.v(x));
                *self.v_mut(x) = res;
                *self.v_mut(0xF) = (!borrow) as u8;
            }
            Opcode::_8XY6(x, y) => {
                if self.shift_vip {
                    *self.v_mut(x) = self.v(y);
                }
                *self.v_mut(0xF) = self.v(x) & 0x01;
                *self.v_mut(x) = self.v(x) >> 1;
            }
            Opcode::_8XYE(x, y) => {
                if self.shift_vip {
                    *self.v_mut(x) = self.v(y);
                }
                *self.v_mut(0xF) = (self.v(x) >> 7) & 0x01;
                *self.v_mut(x) <<= 1;
            }
            Opcode::_ANNN(nnn) => self.i = nnn,
            Opcode::_BNNN(x, nnn) => {
                self.pc = nnn + if self.jump_vip { self.v(0) } else { self.v(x) } as u16
            }
            Opcode::_DXYN(x, y, n) => {
                let vx = self.v(x) & (WIDTH - 1);
                let vy = self.v(y) & (HEIGHT - 1);
                *self.v_mut(0xF) = 0x0;
                for i in 0..n {
                    let byte = self.mem(self.i + i as u16);
                    let sprite_row = BitVec::from_bytes(&[byte]);
                    for bit in 0x0..0x8 {
                        let sprite_bit = sprite_row[bit];
                        if sprite_bit {
                            let x = (vx + bit as u8) % WIDTH;
                            let y = (vy + i) % HEIGHT;
                            let idx = x as usize + WIDTH as usize * y as usize;
                            if self.display.get(idx).unwrap() {
                                *self.v_mut(0xF) = 0x1;
                            }
                            *self.display.get_mut(idx).unwrap() ^= true;
                        }
                    }
                }
            }
            Opcode::NONE => (),
        }
    }

    fn mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn v(&self, addr: u8) -> u8 {
        self.v[addr as usize]
    }

    fn v_mut(&mut self, addr: u8) -> &mut u8 {
        &mut self.v[addr as usize]
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[FONT_RANGE].copy_from_slice(&FONT);
        Self {
            memory,
            display: BitVec::from_elem(DISPLAY_SIZE, false),
            keypad: BitVec::from_elem(16, false),
            pc: MEMORY_START as u16,
            i: 0x0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            shift_vip: false,
            jump_vip: false,
        }
    }
}
