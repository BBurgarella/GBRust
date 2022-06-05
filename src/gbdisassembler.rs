use std::fs;
use std::fmt;
use serde_json::{Result, Value};
use colored::*;

// gets the CPU instructions from a json file
pub fn get_json(fileadress: &str) -> Result<Value>{
    let contents = fs::read_to_string(fileadress)
        .expect("Something went wrong reading the file");

    let v: Value = serde_json::from_str(&contents[..])?;

    return Ok(v)
}

pub struct CPUInstruction {
    pub mnemonic: String,
    pub bytes: u8,
    pub op_code: u8,
    pub operands: Vec<String>,
}

pub fn init_instruction_set(fileadress: &str) -> Vec<CPUInstruction> {
    let v = get_json(fileadress);
    let mut instruction_set: Vec<CPUInstruction> = vec!(instruction_from_json(&v.as_ref().unwrap()["unprefixed"][format!("{:#04X}", 0)], 0));
    for i in 1..0xFF{
        instruction_set.push(instruction_from_json(&v.as_ref().unwrap()["unprefixed"][format!("{:#04X}", i)], i));
    }
    return instruction_set
}

pub fn instruction_from_json(data: &Value, op_code_val: u8) -> CPUInstruction {
    //println!("{}", data);
    // getting the instruction mnemonic
    let end = data["mnemonic"].to_string().len() - 1;
    let mnemo_string = data["mnemonic"].to_string()[1..end].to_string();

    // getting the number of bytes
    let byte_number = data["bytes"].to_string().parse().unwrap();

    // this is a conditional definition of oper_vect
    // the vector of all the statements
    // this is necessary to remove the "null" operands, for example for the NOP instruction
    let mut oper_vect: Vec<String> = if data["operands"][0].to_string() != "null" {
        let end = data["operands"][0]["name"].to_string().len() - 1;
        vec!(data["operands"][0]["name"].to_string()[1..end].to_string())    
    } else {
        vec!("".to_string())   
    };


    let mut count: usize = 1;
    while data["operands"][count].to_string() != "null" {
        let end = data["operands"][count]["name"].to_string().len() - 1;
        oper_vect.push(data["operands"][count]["name"].to_string()[1..end].to_string());
        count += 1;
     }

    // creating the instance
    let instruction = CPUInstruction{mnemonic: mnemo_string, bytes: byte_number, op_code: op_code_val, operands: oper_vect};
    return instruction
}

impl fmt::Display for CPUInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_to_print = format!("{:#04X}: {}", self.op_code, self.mnemonic.blue());
        if &self.operands[0] != "null" {
            for op in &self.operands {
                let op_string = format!("{}",op.yellow());
                string_to_print = format!("{}, {}", string_to_print, op_string);
            }
        }
        write!(f, "{}", string_to_print)
    }
}