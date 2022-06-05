use colored::*;
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

    pub fn dump(&self, begin: u16, end: u16) ->() {
        let mut reader: u16 = begin;
        let mut row: u8;

        let mut return_string = "\nMemory dump: \n     00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".to_string();
        return_string = format!("{}\n{}", return_string, "----------------------------------------------------------------------");
        while reader <= end {
            row = 0;
            let adress_string = format!("{:04X}", &reader);
            let mut data_string: String = format!("{:02X}", self.at(reader as usize));

            let character: char  = if self.at(reader as usize).is_ascii_alphanumeric(){
                self.at(reader as usize) as char
            } else {
                '.'
            };
            let mut char_string: String = format!("|{}", character);
            reader += 1;
            row += 1;

            while row < 16 {
                data_string = format!("{} {}", data_string, format!("{:02X}", self.at(reader as usize)));
                let character: char  = if self.at(reader as usize).is_ascii_alphanumeric(){
                    self.at(reader as usize) as char
                } else {
                    '.'
                };
                char_string = format!("{}{}", char_string, format!("{}", character));
                row += 1;
                reader += 1;
            }
            char_string = format!("{}|", char_string);
            return_string = format!("{}\n{} {} {}",return_string, adress_string.blue(), data_string.red(), char_string.yellow())
        }
        println!("{}", return_string);
    }
}
