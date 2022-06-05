
pub struct Memory {
    pub data: [u8; 0xFFFF]
}

impl Default for Memory {
    fn default() -> Self {
        let array: [u8; 0xFFFF] = [0; 0xFFFF];
        Self {data: array}
    }
}

impl Memory {
    pub fn at(&self, position: usize) -> u8{
        let real_position: usize =  ((position & 0xFF00) >> 8) | (position & 0xFF) << 8;
        return self.data[real_position]
    }

    pub fn set(&mut self, position: usize, data: u8) -> (){
        let real_position: usize =  ((position & 0xFF00) >> 8) | (position & 0xFF) << 8;
        self.data[real_position] = data;
    }
}