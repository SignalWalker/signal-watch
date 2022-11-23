const TFT_SIZE: u16 = 240 * 240;

#[derive(Copy, Clone)]
pub struct Pixel(u16);

impl Default for Pixel {
    fn default() -> Self {
        Self(0)
    }
}

impl Pixel {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        // assuming 5-6-5 for now
        Self(0u16 | (r << 11) | (g << 5) | b)
    }

    pub fn from_bytes(r: u8, g: u8, b: u8) -> Self {
        todo!()
    }

    pub fn r(&self) -> u16 {
        (self.0 & 0b1111_1000_0000_0000) >> 11
    }

    pub fn g(&self) -> u16 {
        (self.0 & 0b0000_0111_1110_0000) >> 5
    }

    pub fn b(&self) -> u16 {
         self.0 & 0b0000_0000_0001_1111
    }
}

pub struct PixelBuf([Pixel; TFT_SIZE as usize]);

impl Default for PixelBuf {
    fn default() -> Self {
        Self([Pixel::default(); TFT_SIZE as usize])
    }
}

impl PixelBuf {
    const LEN_U8: usize = (240 * 240 * 16) / 8;

    pub fn as_u8<'s>(&'s self) -> &'s [u8] {
        unsafe { core::slice::from_raw_parts(self.0.as_ptr().cast(), self.0.len()) }
    }

    pub fn as_u8_mut(&mut self) -> &mut [u8] {
        todo!()
    }

    pub fn as_u16(&self) -> &[u16] {
        todo!()
    }

    pub fn as_u16_mut(&mut self) -> &mut [u16] {
        todo!()
    }

    pub fn as_ptr_mut(&mut self) -> *mut u8 {
        self.0.as_mut_ptr().cast()
    }

    pub fn clear(&mut self, val: u8) {
        unsafe { core::ptr::write_bytes(self.as_ptr_mut(), val, PixelBuf::LEN_U8 - 1) }
    }
}
