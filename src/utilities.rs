/// Utility struct to get bytes from a byte array in Big Endian order
pub struct ByteShifter<'a> {
    bytes: &'a [u8],
    current_byte: usize,
}

impl<'a> ByteShifter<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            current_byte: 0,
        }
    }

    pub fn skip_bytes(&mut self, amount: usize) {
        self.current_byte += amount;
    }

    pub fn read_byte(&mut self) -> u8 {
        self.current_byte += 1;
        self.bytes[self.current_byte - 1]
    }

    pub fn read_2_bytes(&mut self) -> u16 {
        let higher = (self.read_byte() as u16) << 8;
        let lower = self.read_byte() as u16;

        higher | lower
    }

    pub fn read_4_bytes(&mut self) -> u32 {
        let higher = (self.read_2_bytes() as u32) << 16;
        let lower = self.read_2_bytes() as u32;

        higher | lower
    }

    pub fn read_8_bytes(&mut self) -> u64 {
        let higher = (self.read_4_bytes() as u64) << 32;
        let lower = self.read_4_bytes() as u64;

        higher | lower
    }
}
