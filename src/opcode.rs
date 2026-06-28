#[derive(Debug)]
pub enum Opcode {
    _00E0,
    _00EE,
    _1NNN(u16),
    _2NNN(u16),
    _3XNN(u8, u8),
    _4XNN(u8, u8),
    _5XY0(u8, u8),
    _9XY0(u8, u8),
    _6XNN(u8, u8),
    _7XNN(u8, u8),
    _8XY0(u8, u8),
    _8XY1(u8, u8),
    _8XY2(u8, u8),
    _8XY3(u8, u8),
    _8XY4(u8, u8),
    _8XY5(u8, u8),
    _8XY7(u8, u8),
    _8XY6(u8, u8),
    _8XYE(u8, u8),
    _ANNN(u16),
    _BNNN(u8, u16),
    _CXNN(u8, u8),
    _DXYN(u8, u8, u8),
    _EX9E(u8),
    _EXA1(u8),
    _FX07(u8),
    _FX15(u8),
    _FX18(u8),
    _FX1E(u8),
    _FX0A(u8),
    _FX29(u8),
    _FX33(u8),
    NONE,
}

impl Opcode {
    pub fn decode(left: u8, right: u8) -> Self {
        let fetch = (left as u16) << 8 | right as u16;
        let op = (fetch >> 12) as u8;
        let x = ((fetch >> 8) & 0xF) as u8;
        let y = ((fetch >> 4) & 0xF) as u8;
        let n = (fetch & 0xF) as u8;
        let nn = (fetch & 0xFF) as u8;
        let nnn = fetch & 0xFFF;
        match op {
            0x0 => match n {
                0x0 => Self::_00E0,
                0xE => Self::_00EE,
                _ => Self::NONE,
            },
            0x1 => Self::_1NNN(nnn),
            0x2 => Self::_2NNN(nnn),
            0x3 => Self::_3XNN(x, nn),
            0x4 => Self::_4XNN(x, nn),
            0x5 => Self::_5XY0(x, y),
            0x9 => Self::_9XY0(x, y),
            0x6 => Self::_6XNN(x, nn),
            0x7 => Self::_7XNN(x, nn),
            0x8 => match n {
                0x0 => Self::_8XY0(x, y),
                0x1 => Self::_8XY1(x, y),
                0x2 => Self::_8XY2(x, y),
                0x3 => Self::_8XY3(x, y),
                0x4 => Self::_8XY4(x, y),
                0x5 => Self::_8XY5(x, y),
                0x7 => Self::_8XY7(x, y),
                0x6 => Self::_8XY6(x, y),
                0xE => Self::_8XYE(x, y),
                _ => Self::NONE,
            },
            0xA => Self::_ANNN(nnn),
            0xB => Self::_BNNN(x, nnn),
            0xC => Self::_CXNN(x, nn),
            0xD => Self::_DXYN(x, y, n),
            0xE => match n {
                0xE => Self::_EX9E(x),
                0x1 => Self::_EXA1(x),
                _ => Self::NONE,
            }
            0xF => match n {
                0x7 => Self::_FX07(x),
                0x5 => Self::_FX15(x),
                0x8 => Self::_FX18(x),
                0xE => Self::_FX1E(x),
                0xA => Self::_FX0A(x),
                0x9 => Self::_FX29(x),
                0x3 => Self::_FX33(x),
                _ => Self::NONE,
            }
            _ => Self::NONE,
        }
    }
}
