use colored::*;
pub struct Memory {
    pub data: Vec<u8>,
    pub offset: u16,
}

impl Default for Memory {
    fn default() -> Self {
        let array =vec![0; 0xFFFF];
        Self {data: array, offset: 0x0000}
    }
}

impl Memory {
    pub fn at(&self, position: usize) -> u8{
        let local_position = position - self.offset as usize;
        //println!("{:#04X}", local_position);
        //let real_position: usize =  ((local_position & 0xFF00) >> 8) | (local_position & 0xFF) << 8;
        //println!("{:#04X}", real_position);
        //println!("{:#04X}", self.offset);
        return self.data[local_position]
    }

    pub fn set(&mut self, position: usize, data: u8) -> (){
        let local_position = position - self.offset as usize;
        //let real_position: usize =  ((position & 0xFF00) >> 8) | (position & 0xFF) << 8;
        self.data[local_position] = data;
    }

    pub fn dump(&self, begin: u16, end: u16) ->() {
        let mut reader: u16 = begin - begin%0x10;
        let mut row: u8;

        let mut return_string = "\nMemory dump: \n     00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".to_string();
        return_string = format!("{}\n{}", return_string, "----------------------------------------------------------------------");
        while reader <= end {
            row = 0;
            let adress_string = format!("{:04X}", &reader);
            let mut data_string: String = format!("{:02X}", self.at(reader as usize));
            if reader == begin {
                data_string = format!("{}", data_string.on_bright_green());
            }

            let character: char  = if self.at(reader as usize).is_ascii_alphanumeric(){
                self.at(reader as usize) as char
            } else {
                '.'
            };
            let mut char_string: String = format!("|{}", character);
            reader += 1;
            row += 1;

            while row < 16 {
                let new_data: String;
                if reader == begin {
                    new_data = format!("{}", format!("{:02X}", self.at(reader as usize)).on_bright_green());
                } else {
                    new_data = format!("{}", format!("{:02X}", self.at(reader as usize)));
                }

                data_string = format!("{} {}", data_string, new_data);
            
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
