use crate::FloatType;
#[derive(Clone, Copy)]
pub struct Pixel {
    byte: u8,
    deepth: FloatType,
}
impl Pixel {
    pub fn new(byte:u8)->Self{
        Self{
            byte,
            deepth:FloatType::MAX,
        }
    }
    pub fn get_byte(&self) -> u8 {
        self.byte
    }
    pub fn set_byte(&mut self, byte: u8) {
        self.byte = byte;
    }
    pub fn get_deepth(&self) -> FloatType {
        self.deepth
    }
    pub fn set_deepth(&mut self, deepth: FloatType) {
        self.deepth = deepth;
    }
}
impl Default for Pixel{
    fn default() -> Self {
        Self { 
            byte: b' ', 
            deepth: FloatType::MAX, 
        }
    }
}