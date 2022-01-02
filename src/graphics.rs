pub fn data(pos:usize)->u8 {
    let data:[u8;4096] = [
    0x00,0x00,0x3F,0x3F,0x00,0x00,0x00,0xC0,0x02,0x80,0x02,0x80,0x02,0x80,0x55,0x55,
    0x3C,0x93,0x1F,0x1F,0x00,0x00,0xCC,0x0C,0x33,0x00,0x03,0xC0,0x03,0xC0,0x55,0x55,
    0x90,0xBB,0x15,0x15,0x00,0x00,0x00,0x00,0x08,0x23,0x08,0x20,0x09,0xA0,0x41,0x41,
    0x3C,0xBF,0x00,0x00,0x00,0x00,0x0C,0xCC,0x0C,0x70,0x0C,0x70,0x0D,0x70,0x51,0x51,
    0x09,0x97,0xF3,0xF3,0x00,0x30,0x30,0x00,0x01,0x08,0x21,0x08,0x2B,0xD8,0x71,0x71,
    0x09,0x93,0xF1,0xF1,0x0C,0x00,0x00,0x30,0x00,0x0C,0x30,0xCC,0x37,0x5C,0x55,0x55,
    0x3C,0x93,0x51,0x51,0x30,0xC0,0xC0,0x03,0x84,0x12,0x87,0x92,0x9E,0xBE,0x55,0x55,
    0x00,0x00,0x00,0x00,0x03,0x0C,0x00,0x00,0xC0,0x03,0xE1,0x47,0xF7,0xDB,0x55,0x55,
    0x00,0x00,0x3F,0x3F,0x00,0xC0,0x00,0x00,0x80,0x01,0x83,0x29,0x97,0x79,0x55,0x55,
    0x99,0x00,0x1F,0x1F,0x0C,0x00,0xC0,0x0C,0xC4,0x03,0xC4,0x43,0xDE,0xEB,0x55,0x55,
    0x99,0x00,0x15,0x15,0x30,0x30,0x00,0x00,0x00,0x14,0x28,0x34,0x27,0xB4,0x41,0x41,
    0x3C,0x00,0x00,0x00,0x03,0xC0,0x30,0xC3,0x31,0x0C,0x31,0x4C,0x36,0x6C,0x51,0x51,
    0x24,0x00,0xF3,0xF3,0x00,0x00,0x03,0x00,0x08,0x90,0x0B,0x10,0x0B,0xD0,0x71,0x71,
    0x24,0x00,0xF1,0xF1,0x00,0x00,0xC0,0x0C,0x0C,0x03,0x0C,0xB0,0x0D,0xB0,0x55,0x55,
    0x24,0x00,0x51,0x51,0x00,0x00,0x0C,0x00,0x32,0x40,0x02,0x40,0x02,0x40,0x55,0x55,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x55,0x55,
    0x3F,0x3F,0x00,0x00,0x0F,0xC0,0x08,0x22,0x02,0x80,0xF0,0xF0,0x0F,0x0F,0xF0,0x0F,
    0x1F,0x1F,0x04,0x10,0x37,0xB0,0xA0,0x82,0x03,0xC0,0x00,0x00,0x00,0x00,0x3F,0xFC,
    0x15,0x15,0x05,0x50,0xED,0xFC,0x26,0x24,0x08,0x20,0x00,0x00,0x00,0x00,0xF7,0xDF,
    0x00,0x00,0x11,0x44,0x79,0xBF,0xA2,0xA2,0x0C,0x30,0x00,0x00,0x00,0x00,0xF7,0xDF,
    0xF3,0xF3,0x11,0x44,0x56,0x7F,0x8A,0x8A,0x20,0x08,0x0F,0x0F,0xF0,0xF0,0xFF,0xFF,
    0xF1,0xF1,0x05,0x50,0x65,0x67,0x18,0x98,0x30,0x0C,0x00,0x00,0x00,0x00,0x3C,0x3C,
    0x51,0x51,0x01,0x40,0x5A,0x5F,0x69,0x4A,0x80,0x02,0x00,0x00,0x00,0x00,0x0F,0x30,
    0x00,0x00,0x05,0x50,0x55,0x67,0x8A,0xA4,0xC0,0x03,0x00,0x00,0x00,0x00,0x05,0x50,
    0x00,0x00,0x13,0xC4,0x45,0x56,0x2A,0x6A,0x80,0x01,0xF0,0xF0,0x0F,0x0F,0xA5,0x58,
    0x00,0x00,0x31,0x4C,0x51,0x66,0x92,0xA4,0xC0,0x03,0x00,0x00,0x00,0x00,0x85,0x52,
    0x00,0x30,0x03,0xC0,0x45,0x55,0x98,0x62,0x20,0x04,0x00,0x00,0x00,0x00,0x05,0x58,
    0x00,0x03,0x01,0x40,0x51,0x55,0x1A,0x26,0x30,0x0C,0x00,0x00,0x00,0x00,0x3F,0xFC,
    0x05,0x32,0x0B,0xE0,0x54,0x54,0x88,0x88,0x08,0x10,0x0F,0x0F,0xF0,0xF0,0x3C,0x3C,
    0x05,0x24,0x08,0x20,0x15,0x54,0xA0,0xA2,0x0C,0x30,0x00,0x00,0x00,0x00,0x30,0x0C,
    0x07,0x10,0x08,0x20,0x04,0x90,0x89,0x20,0x02,0x40,0x00,0x00,0x00,0x00,0xF0,0x0C,
    0x07,0xE1,0x3C,0x3C,0x01,0x40,0x20,0x88,0x03,0xC0,0x00,0x00,0x00,0x00,0x00,0x0F,
    0x00,0x00,0x55,0x55,0xFF,0x3F,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0x3F,0x3F,0x3F,
    0x00,0x00,0x40,0x01,0xDF,0x3F,0x00,0x00,0x00,0x00,0x00,0x00,0xDF,0x3F,0x1F,0x1F,
    0x00,0x00,0x40,0x01,0xD5,0x35,0x00,0x00,0x00,0x00,0x30,0xCC,0xD5,0x35,0x1D,0x17,
    0x00,0x00,0x40,0x01,0xC0,0x30,0x00,0x00,0x00,0x30,0x03,0x10,0xC0,0x30,0x0C,0x03,
    0x00,0x00,0x40,0x01,0xF3,0xFF,0x00,0x30,0x03,0x20,0x32,0x2C,0xF3,0xFF,0xF3,0xF3,
    0x00,0x00,0x40,0x01,0xF3,0xFD,0x0C,0x00,0x38,0x0C,0x24,0x08,0xF3,0xFD,0xF1,0xF1,
    0x00,0x00,0x40,0x01,0x53,0x5D,0x30,0xC0,0x20,0xB8,0x10,0x64,0x53,0x5D,0x71,0xD1,
    0x00,0x00,0x40,0x01,0x03,0x0C,0x03,0x0C,0x32,0x08,0xE1,0x04,0x03,0x0C,0x30,0xC0,
    0x00,0x00,0x40,0x01,0xBC,0xBC,0x00,0xC0,0x00,0xB0,0x00,0xA0,0x55,0x00,0x00,0x00,
    0x00,0x00,0x40,0x01,0xBC,0xBC,0x0C,0x00,0x38,0x80,0xE4,0x8C,0x55,0xF0,0x00,0x00,
    0x00,0x00,0x40,0x01,0xBC,0xBC,0x30,0x30,0x22,0x20,0x11,0x20,0x41,0x00,0x00,0x00,
    0x00,0x00,0x40,0x01,0xBC,0xBC,0x03,0xC0,0x0E,0x8C,0x09,0x88,0x51,0x00,0x00,0x00,
    0x00,0x00,0x40,0x01,0x1F,0xFF,0x00,0x00,0x0C,0xC0,0xC8,0x8C,0x71,0x00,0x00,0x00,
    0x00,0x00,0x40,0x01,0x00,0xBC,0x00,0x00,0x00,0x00,0x03,0xC0,0x55,0x0F,0x00,0x00,
    0x00,0x00,0x40,0x01,0x00,0xBC,0x00,0x00,0x00,0x00,0x30,0x00,0x55,0x00,0x00,0x00,
    0x00,0x00,0x55,0x55,0x00,0xBC,0x00,0x00,0x00,0x00,0x00,0x00,0x55,0x00,0x00,0x00,
    0xFF,0xBF,0xFF,0xBF,0xFE,0x2F,0xFE,0x2F,0xF8,0x0B,0xF8,0x0B,0xFE,0x2F,0xFE,0x2F,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xBF,0xFF,0xBF,0xFE,0x2F,0xFE,0x2F,0xFF,0xBF,0xFF,0xBF,
    0xBF,0xFF,0xBF,0xFF,0xFF,0xFE,0xFF,0xFE,0xFF,0xBF,0xFF,0xBF,0xFF,0xFF,0xFF,0xFF,
    0x2F,0xFE,0x2F,0xFE,0xBF,0xF8,0xBF,0xF8,0xFF,0xFE,0xFF,0xFE,0xBF,0xFF,0xBF,0xFF,
    0x2F,0xFE,0x2F,0xFE,0x2F,0xE8,0x2F,0xE8,0xBF,0xF8,0xBF,0xF8,0x2F,0xFE,0x2F,0xFE,
    0x2F,0xFE,0x2F,0xFE,0xBF,0xF8,0xBF,0xF8,0xFF,0xFE,0xFF,0xFE,0xBF,0xFF,0xBF,0xFF,
    0xBF,0xFF,0xBF,0xFF,0xFF,0xFE,0xFF,0xFE,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0xBF,0xFE,0xBF,0xFE,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x2F,0xF8,0x2F,0xF8,0xBF,0xFE,0xBF,0xFE,0xFF,0xFF,0xFF,0xFF,0xAF,0xFE,0xAF,0xFE,
    0x2F,0xF8,0x2F,0xF8,0x2F,0xF8,0x2F,0xF8,0xFF,0xFF,0xFF,0xFF,0x2F,0xF8,0x2F,0xF8,
    0x2F,0xF8,0x2F,0xF8,0xBF,0xFE,0xBF,0xFE,0xFF,0xFF,0xFF,0xFF,0xAF,0xFE,0xAF,0xFE,
    0xBF,0xFE,0xBF,0xFE,0xFF,0xFF,0xFF,0xFF,0xFF,0xBF,0xFF,0xBF,0xFF,0xFF,0xFF,0xFF,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xBF,0xFF,0xBF,0xFE,0x2F,0xFE,0x2F,0xFF,0xBF,0xFF,0xBF,
    0xFF,0xFF,0xFF,0xFF,0xFE,0x2F,0xFE,0x2F,0xF8,0x0B,0xF8,0x0B,0xFE,0x2F,0xFE,0x2F,
    0xAA,0xAA,0xAA,0xAA,0x00,0x00,0x00,0x00,0xAA,0xAA,0xAA,0xAA,0xFF,0xFF,0xFF,0xFF,
    0xAA,0xAA,0xAA,0xAA,0x00,0x00,0x00,0x00,0xAA,0xAA,0xAA,0xAA,0xFF,0xFF,0xFF,0xFF,
    0x80,0x02,0x80,0x02,0x2A,0xA8,0x2A,0xA8,0xBF,0xFE,0xBF,0xFE,0xEA,0xAB,0xEA,0xAB,
    0x80,0x02,0x80,0x02,0x2A,0xA8,0x2A,0xA8,0xBF,0xFE,0xBF,0xFE,0xEA,0xAB,0xEA,0xAB,
    0x8A,0xA2,0x8A,0xA2,0x2F,0xF8,0x2F,0xF8,0xBA,0xAE,0xBA,0xAE,0xE0,0x0B,0xE0,0x0B,
    0x8A,0xA2,0x8A,0xA2,0x2F,0xF8,0x2F,0xF8,0xBA,0xAE,0xBA,0xAE,0xE0,0x0B,0xE0,0x0B,
    0x8B,0xE2,0x8B,0xE2,0x2E,0xB8,0x2E,0xB8,0xB8,0x2E,0xB8,0x2E,0xE2,0x8B,0xE2,0x8B,
    0x8B,0xE2,0x8B,0xE2,0x2E,0xB8,0x2E,0xB8,0xB8,0x2E,0xB8,0x2E,0xE2,0x8B,0xE2,0x8B,
    0x8B,0xE2,0x8B,0xE2,0x2E,0xB8,0x2E,0xB8,0xB8,0x2E,0xB8,0x2E,0xE2,0x8B,0xE2,0x8B,
    0x8B,0xE2,0x8B,0xE2,0x2E,0xB8,0x2E,0xB8,0xB8,0x2E,0xB8,0x2E,0xE2,0x8B,0xE2,0x8B,
    0x8A,0xA2,0x8A,0xA2,0x2F,0xF8,0x2F,0xF8,0xBA,0xAE,0xBA,0xAE,0xE0,0x0B,0xE0,0x0B,
    0x8A,0xA2,0x8A,0xA2,0x2F,0xF8,0x2F,0xF8,0xBA,0xAE,0xBA,0xAE,0xE0,0x0B,0xE0,0x0B,
    0x80,0x02,0x80,0x02,0x2A,0xA8,0x2A,0xA8,0xBF,0xFE,0xBF,0xFE,0xEA,0xAB,0xEA,0xAB,
    0x80,0x02,0x80,0x02,0x2A,0xA8,0x2A,0xA8,0xBF,0xFE,0xBF,0xFE,0xEA,0xAB,0xEA,0xAB,
    0xAA,0xAA,0xAA,0xAA,0x00,0x00,0x00,0x00,0xAA,0xAA,0xAA,0xAA,0xFF,0xFF,0xFF,0xFF,
    0xAA,0xAA,0xAA,0xAA,0x00,0x00,0x00,0x00,0xAA,0xAA,0xAA,0xAA,0xFF,0xFF,0xFF,0xFF,
    0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,
    0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,
    0x0B,0xF0,0x0B,0xF0,0x0A,0xB0,0x0B,0xF0,0x08,0x30,0x08,0x30,0x0B,0xF0,0x0A,0xB0,
    0x0F,0xE0,0x0E,0xA0,0x0F,0xE0,0x0C,0x20,0x0C,0x20,0x0F,0xE0,0x0E,0xA0,0x0F,0xE0,
    0x2A,0xAC,0x2F,0xFC,0x20,0x0C,0x20,0x0C,0x2F,0xFC,0x2A,0xAC,0x2F,0xFC,0x2F,0xFC,
    0x3F,0xF8,0x30,0x08,0x30,0x08,0x3F,0xF8,0x3A,0xA8,0x3F,0xF8,0x3F,0xF8,0x3A,0xA8,
    0x80,0x03,0x80,0x03,0xBF,0xFF,0xAA,0xAB,0xBF,0xFF,0xBF,0xFF,0xAA,0xAB,0xBF,0xFF,
    0xC0,0x02,0xFF,0xFE,0xEA,0xAA,0xFF,0xFE,0xFF,0xFE,0xEA,0xAA,0xFF,0xFE,0xC0,0x02,
    0xBF,0xFF,0xAA,0xAB,0xBF,0xFF,0xBF,0xFF,0xAA,0xAB,0xBF,0xFF,0x80,0x03,0x80,0x03,
    0xEA,0xAA,0xFF,0xFE,0xFF,0xFE,0xEA,0xAA,0xFF,0xFE,0xC0,0x02,0xC0,0x02,0xFF,0xFE,
    0x2F,0xFC,0x2F,0xFC,0x2A,0xAC,0x2F,0xFC,0x20,0x0C,0x20,0x0C,0x2F,0xFC,0x2A,0xAC,
    0x3F,0xF8,0x3A,0xA8,0x3F,0xF8,0x30,0x08,0x30,0x08,0x3F,0xF8,0x3A,0xA8,0x3F,0xF8,
    0x0A,0xB0,0x0B,0xF0,0x08,0x30,0x08,0x30,0x0B,0xF0,0x0A,0xB0,0x0B,0xF0,0x0B,0xF0,
    0x0F,0xE0,0x0C,0x20,0x0C,0x20,0x0F,0xE0,0x0E,0xA0,0x0F,0xE0,0x0F,0xE0,0x0E,0xA0,
    0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0xC0,0x02,0xC0,0x02,0x30,0x08,0x0C,0x20,0x0C,0x20,0x30,0x08,0xC0,0x02,0xC0,0x02,
    0x80,0x03,0x80,0x03,0x30,0x08,0x08,0x30,0x08,0x30,0x30,0x08,0x80,0x03,0x80,0x03,
    0xF0,0x0B,0xF0,0x0B,0x20,0x0C,0x0C,0x20,0x0C,0x20,0x20,0x0C,0xF0,0x0B,0xF0,0x0B,
    0xE0,0x0F,0xE0,0x0F,0x20,0x0C,0x08,0x30,0x08,0x30,0x20,0x0C,0xE0,0x0F,0xE0,0x0F,
    0xAC,0x2A,0xAC,0x2A,0x2C,0x28,0x08,0x20,0x08,0x20,0x2C,0x28,0xAC,0x2A,0xAC,0x2A,
    0xF8,0x3F,0xF8,0x3F,0x38,0x3C,0x0C,0x30,0x0C,0x30,0x38,0x3C,0xF8,0x3F,0xF8,0x3F,
    0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,0x03,0x80,
    0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,0x02,0xC0,
    0xFF,0xBF,0xFF,0xBF,0x3F,0xBC,0x03,0x80,0x03,0x80,0x3F,0xBC,0xFF,0xBF,0xFF,0xBF,
    0xAA,0xEA,0xAA,0xEA,0x2A,0xE8,0x02,0xC0,0x02,0xC0,0x2A,0xE8,0xAA,0xEA,0xAA,0xEA,
    0xFC,0x2F,0xFC,0x2F,0x3C,0x2C,0x08,0x20,0x08,0x20,0x3C,0x2C,0xFC,0x2F,0xFC,0x2F,
    0xF8,0x3F,0xF8,0x3F,0x38,0x3C,0x0C,0x30,0x0C,0x30,0x38,0x3C,0xF8,0x3F,0xF8,0x3F,
    0xB0,0x0A,0xB0,0x0A,0x30,0x08,0x08,0x20,0x08,0x20,0x30,0x08,0xB0,0x0A,0xB0,0x0A,
    0xE0,0x0F,0xE0,0x0F,0x20,0x0C,0x0C,0x30,0x0C,0x30,0x20,0x0C,0xE0,0x0F,0xE0,0x0F,
    0x80,0x03,0x80,0x03,0x20,0x08,0x08,0x20,0x08,0x20,0x20,0x08,0x80,0x03,0x80,0x03,
    0xC0,0x03,0xC0,0x03,0x30,0x0C,0x0C,0x30,0x0C,0x30,0x30,0x0C,0xC0,0x03,0xC0,0x03,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x01,0x40,0x01,0x40,0x01,0x40,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x40,0x01,0x40,
    0x05,0x50,0x05,0x50,0x05,0x50,0x01,0x40,0x01,0x40,0x01,0x40,0x05,0x50,0x05,0x50,
    0x11,0x50,0x11,0x50,0x11,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x11,0x50,0x11,0x50,
    0x11,0x50,0x11,0x50,0x11,0x50,0x11,0x50,0x11,0x50,0x11,0x50,0x11,0x50,0x11,0x50,
    0x05,0x50,0x05,0x50,0x05,0x50,0x11,0x50,0x11,0x50,0x11,0x50,0x05,0x50,0x05,0x50,
    0x01,0x40,0x01,0x40,0x01,0x40,0x05,0x50,0x05,0x50,0x05,0x50,0x01,0x40,0x01,0x40,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x0D,0x40,0x0D,0x40,0x0D,0x40,0x0D,0x40,0x0D,0x40,0x0D,0x40,0x0D,0x40,0x0D,0x40,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x0B,0xE8,0x0B,0xE8,0x0B,0xE0,0x0B,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x0B,0xC0,
    0x20,0x03,0x20,0x03,0x08,0x2B,0x08,0xA8,0x02,0x80,0x0A,0x80,0x0A,0x80,0x08,0xA8,
    0x20,0x03,0x20,0x03,0x08,0x03,0x08,0x2C,0x02,0xB0,0x0A,0x80,0x0A,0x80,0x08,0x2C,
    0xF0,0x00,0xF0,0x00,0x3C,0x00,0x3C,0x0C,0x0F,0x30,0x03,0xC0,0x0F,0xF0,0x3C,0x0C,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x01,0x40,0x01,0x40,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x40,0x01,0x40,0x01,0x40,
    0x05,0x50,0x05,0x50,0x01,0x40,0x01,0x40,0x01,0x40,0x05,0x50,0x05,0x50,0x05,0x50,
    0x05,0x44,0x05,0x44,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x44,0x05,0x44,0x05,0x44,
    0x05,0x44,0x05,0x44,0x05,0x44,0x05,0x44,0x05,0x44,0x05,0x44,0x05,0x44,0x05,0x44,
    0x05,0x50,0x05,0x50,0x05,0x44,0x05,0x44,0x05,0x44,0x05,0x50,0x05,0x50,0x05,0x50,
    0x01,0x40,0x01,0x40,0x05,0x50,0x05,0x50,0x05,0x50,0x01,0x40,0x01,0x40,0x01,0x40,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x01,0x70,0x01,0x70,0x01,0x70,0x01,0x70,0x01,0x70,0x01,0x70,0x01,0x70,0x01,0x70,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x03,0xE0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xE0,0x0B,0xE0,0x2B,0xE0,0x2B,0xE0,
    0x2A,0x20,0x02,0xA0,0x02,0xA0,0x02,0x80,0x2A,0x20,0xE8,0x20,0xC0,0x08,0xC0,0x08,
    0x38,0x20,0x02,0xA0,0x02,0xA0,0x0E,0x80,0x38,0x20,0xC0,0x20,0xC0,0x08,0xC0,0x08,
    0x30,0x3C,0x0F,0xF0,0x03,0xC0,0x0C,0xF0,0x30,0x3C,0x00,0x3C,0x00,0x0F,0x00,0x0F,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x15,0x54,0x15,0x54,0x15,0x54,0x15,0x54,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,
    0x11,0x44,0x15,0x54,0x15,0x54,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,
    0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,
    0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,
    0x3C,0x20,0x3C,0x20,0x3C,0x20,0x3C,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,
    0x00,0x3C,0x00,0x3C,0x00,0x3C,0x00,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x15,0x54,0x15,0x54,0x15,0x54,0x15,0x54,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,
    0x11,0x44,0x15,0x54,0x15,0x54,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,
    0x31,0x4C,0x31,0x4C,0x31,0x4C,0x31,0x4C,0x31,0x4C,0x31,0x4C,0x31,0x4C,0x31,0x4C,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,
    0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,
    0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,
    0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,0x04,0x10,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,
    0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,0x05,0x50,
    0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,0x13,0xC4,
    0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,0x0D,0x70,
    0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,0x03,0xC0,
    0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,0x01,0x40,
    0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,0x0B,0xE0,
    0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,
    0x3C,0x20,0x3C,0x20,0x3C,0x20,0x3C,0x20,0x08,0x20,0x08,0x20,0x08,0x20,0x08,0x20,
    0x00,0x3C,0x00,0x3C,0x00,0x3C,0x00,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,0x3C,
    0xFF,0xFF,0x3F,0x3F,0x3F,0x3F,0x3F,0x3F,0x55,0x55,0x41,0x41,0x71,0x71,0x55,0x55,
    0xDF,0xDF,0x3F,0x3F,0x1F,0x1F,0x1F,0x1F,0x55,0x55,0x51,0x51,0x55,0x55,0x55,0x55,
    0xD5,0xD5,0x35,0x35,0x1D,0x1D,0x17,0x17,0x41,0x41,0x71,0x71,0x55,0x55,0x55,0x55,
    0xC0,0xC0,0x30,0x30,0x0C,0x0C,0x03,0x03,0x51,0x51,0x55,0x55,0x55,0x55,0x55,0x55,
    0xF3,0xF3,0xFF,0xFF,0xF3,0xF3,0xF3,0xF3,0x71,0x71,0x55,0x55,0x55,0x55,0x41,0x41,
    0xF3,0xF3,0xFD,0xFD,0xF1,0xF1,0xF1,0xF1,0x55,0x55,0x55,0x55,0x55,0x55,0x51,0x51,
    0x53,0x53,0x5D,0x5D,0x71,0x71,0xD1,0xD1,0x55,0x55,0x55,0x55,0x41,0x41,0x71,0x71,
    0x03,0x03,0x0C,0x0C,0x30,0x30,0xC0,0xC0,0x55,0x55,0x55,0x55,0x51,0x51,0x55,0x55,
    0xFF,0xFF,0x3F,0x3F,0x3F,0x3F,0x3F,0x3F,0x55,0x55,0x41,0x41,0x71,0x71,0x55,0x55,
    0xDF,0xDF,0x3F,0x3F,0x1F,0x1F,0x1F,0x1F,0x55,0x55,0x51,0x51,0x55,0x55,0x55,0x55,
    0xD5,0xD5,0x35,0x35,0x1D,0x1D,0x17,0x17,0x41,0x41,0x71,0x71,0x55,0x55,0x55,0x55,
    0xC0,0xC0,0x30,0x30,0x0C,0x0C,0x03,0x03,0x51,0x51,0x55,0x55,0x55,0x55,0x55,0x55,
    0xF3,0xF3,0xFF,0xFF,0xF3,0xF3,0xF3,0xF3,0x71,0x71,0x55,0x55,0x55,0x55,0x41,0x41,
    0xF3,0xF3,0xFD,0xFD,0xF1,0xF1,0xF1,0xF1,0x55,0x55,0x55,0x55,0x55,0x55,0x51,0x51,
    0x53,0x53,0x5D,0x5D,0x71,0x71,0xD1,0xD1,0x55,0x55,0x55,0x55,0x41,0x41,0x71,0x71,
    0x03,0x03,0x0C,0x0C,0x30,0x30,0xC0,0xC0,0x55,0x55,0x55,0x55,0x51,0x51,0x55,0x55,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x3F,0xF0,0x03,0xC0,0x3F,0xF0,0x3F,0xFC,0xF0,0x00,0xFF,0xFC,0x3F,0xF0,0xFF,0xFC,
    0xFC,0x3C,0x0F,0xC0,0xF0,0xFC,0x00,0xF0,0xF0,0x00,0xFC,0x00,0xFC,0x00,0xC0,0xFC,
    0xFC,0x3C,0x0F,0xC0,0x03,0xF0,0x03,0xC0,0xF3,0xF0,0xFF,0xF0,0xFF,0xF0,0x03,0xF0,
    0xFC,0x3C,0x03,0xC0,0x0F,0xC0,0x00,0xF0,0xFF,0xFC,0x00,0x3C,0xFC,0x3C,0x0F,0xC0,
    0xFC,0x3C,0x03,0xC0,0x3F,0x00,0xFC,0x3C,0x03,0xF0,0xFC,0x3C,0xFC,0x3C,0x3F,0x00,
    0xFF,0xFC,0x3F,0xFC,0xFF,0xFC,0xFF,0xFC,0x03,0xF0,0xFF,0xFC,0xFF,0xFC,0xFC,0x00,
    0x3F,0xF0,0x3F,0xFC,0xFF,0xFC,0x3F,0xF0,0x03,0xF0,0x3F,0xF0,0x3F,0xF0,0xFC,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF,0xFF,0x00,0x00,0x00,0x00,0x00,0x00,
    0x3F,0xF0,0x3F,0xF0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0C,
    0xFC,0x3C,0xFC,0x3C,0x03,0xC0,0x03,0xC0,0xFF,0xFF,0x3C,0x3C,0x00,0x00,0x00,0x3C,
    0x3F,0xF0,0xFC,0x3C,0x03,0xC0,0x03,0xC0,0x00,0x00,0x0F,0xF0,0x00,0x00,0x00,0xF0,
    0xFC,0x3C,0x3F,0xFC,0x00,0x00,0x00,0x00,0xFF,0xFF,0xFF,0xFF,0x00,0x00,0x03,0xC0,
    0xFC,0x3C,0x03,0xF0,0x03,0xC0,0x03,0xC0,0x00,0x00,0x0F,0xF0,0x03,0xC0,0x0F,0x00,
    0xFF,0xFC,0x0F,0xC0,0x03,0xC0,0x03,0xC0,0xFF,0xFF,0x3C,0x3C,0x03,0xC0,0x3C,0x00,
    0x3F,0xF0,0x3F,0x00,0x00,0x00,0x0F,0x00,0x00,0x00,0x00,0x00,0x0F,0x00,0xF0,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x0F,0xC0,0xFF,0xF0,0x3F,0xF0,0xFF,0xC0,0xFF,0xFC,0xFF,0xFC,0x3F,0xFC,
    0x00,0x00,0x3F,0xF0,0xFC,0x3C,0xFC,0x3C,0xFC,0xF0,0xFC,0x00,0xFC,0x00,0xFC,0x00,
    0x00,0x00,0xFC,0x3C,0xFF,0xF0,0xFC,0x00,0xFC,0x3C,0xFF,0xF0,0xFF,0xF0,0xFC,0x00,
    0x00,0x00,0xFC,0x3C,0xFC,0x3C,0xFC,0x00,0xFC,0x3C,0xFC,0x00,0xFC,0x00,0xFC,0xFC,
    0x00,0x00,0xFF,0xFC,0xFC,0x3C,0xFC,0x3C,0xFC,0xF0,0xFC,0x00,0xFC,0x00,0xFC,0x3C,
    0x00,0x00,0xFC,0x3C,0xFF,0xFC,0xFF,0xFC,0xFF,0xF0,0xFF,0xFC,0xFC,0x00,0xFF,0xFC,
    0x00,0x00,0xFC,0x3C,0xFF,0xF0,0x3F,0xF0,0xFF,0xC0,0xFF,0xFC,0xFC,0x00,0x3F,0xFC,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xFC,0x3C,0x3F,0xF0,0x00,0x3C,0xFC,0x3C,0xFC,0x00,0xF0,0x3C,0xFC,0x3C,0x3F,0xF0,
    0xFC,0x3C,0x0F,0xC0,0x00,0x3C,0xFC,0xF0,0xFC,0x00,0xFC,0xFC,0xFF,0x3C,0xF0,0x3C,
    0xFF,0xFC,0x0F,0xC0,0x00,0x3C,0xFF,0xC0,0xFC,0x00,0xFF,0xFC,0xFF,0xFC,0xF0,0x3C,
    0xFC,0x3C,0x0F,0xC0,0x00,0x3C,0xFF,0xC0,0xFC,0x00,0xF3,0x3C,0xFF,0xFC,0xF0,0x3C,
    0xFC,0x3C,0x0F,0xC0,0xFC,0x3C,0xFC,0xF0,0xFC,0x00,0xF0,0x3C,0xFF,0xFC,0xF0,0x3C,
    0xFC,0x3C,0x3F,0xF0,0x3F,0xFC,0xFC,0x3C,0xFF,0xFC,0xF0,0x3C,0xFC,0xFC,0xFF,0xFC,
    0xFC,0x3C,0x3F,0xF0,0x0F,0xF0,0xFC,0x3C,0xFF,0xFC,0xF0,0x3C,0xFC,0x3C,0x3F,0xF0,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xFF,0xF0,0x0F,0xF0,0xFF,0xF0,0x3F,0xF0,0xFF,0xFC,0xFC,0x3C,0xFC,0x3C,0xF0,0x3C,
    0xFC,0x3C,0xFC,0x3C,0xFC,0x3C,0xFC,0x00,0x0F,0xC0,0xFC,0x3C,0xFC,0x3C,0xF0,0x3C,
    0xFC,0x3C,0xFC,0x3C,0xFC,0x3C,0x3F,0xF0,0x0F,0xC0,0xFC,0x3C,0xFC,0x3C,0xF3,0x3C,
    0xFF,0xFC,0xFC,0x3C,0xFF,0xF0,0x00,0x3C,0x0F,0xC0,0xFC,0x3C,0xFC,0x3C,0xFF,0xFC,
    0xFF,0xF0,0xFC,0xF0,0xFF,0xF0,0x00,0x3C,0x0F,0xC0,0xFC,0x3C,0xFF,0xFC,0xFF,0xFC,
    0xFC,0x00,0x3F,0xFC,0xFC,0x3C,0xFF,0xFC,0x0F,0xC0,0xFF,0xFC,0x3F,0xF0,0xFC,0xFC,
    0xFC,0x00,0x0F,0x3C,0xFC,0x3C,0xFF,0xF0,0x0F,0xC0,0xFF,0xFC,0x0F,0xC0,0xF0,0x3C,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xFC,0x3C,0xF0,0x3C,0xFF,0xFC,0x00,0xF0,0x03,0x00,0x0F,0x00,0x0C,0x30,0x00,0x00,
    0xFC,0x3C,0xF0,0x3C,0x03,0xF0,0x03,0xC0,0x0C,0xC0,0x03,0xC0,0x0F,0xF0,0x00,0x00,
    0x0F,0xF0,0x3F,0xF0,0x0F,0xC0,0x03,0xC0,0x3F,0xF0,0x03,0xC0,0x33,0xCC,0x00,0x00,
    0x0F,0xF0,0x0F,0xC0,0x3F,0x00,0x03,0xC0,0xC0,0x0C,0x03,0xC0,0x33,0xCC,0x00,0x00,
    0xFC,0x3C,0x0F,0xC0,0xFC,0x00,0x03,0xC0,0x3F,0xF0,0x03,0xC0,0x0F,0xF0,0x00,0x00,
    0xFC,0x3C,0x0F,0xC0,0xFF,0xFC,0x03,0xC0,0x0C,0xC0,0x03,0xC0,0x03,0xC0,0xFF,0xFF,
    0xFC,0x3C,0x0F,0xC0,0xFF,0xFC,0x00,0xF0,0x03,0x00,0x0F,0x00,0x0F,0xF0,0x00,0x00,];
    data[pos]
}