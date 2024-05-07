use crate::bytecode::opcode;
pub const STACK_LIMIT: usize = 1024;

pub fn opcode_to_string(opcode: usize) -> String {
  match opcode {
    opcode::OPCODE_HALF => "HALF".to_string(),
    opcode::OPCODE_CONST => "CONST".to_string(),
    opcode::OPCODE_ADD => "ADD".to_string(),
    opcode::OPCODE_SUB => "SUB".to_string(),
    opcode::OPCODE_MUL => "MUL".to_string(),
    opcode::OPCODE_DIV => "DIV".to_string(),
    opcode::OPCODE_EQ => "EQ".to_string(),
    opcode::OPCODE_JUMP => "JUMP".to_string(),
    opcode::OPCODE_JUMP_IF_FALSE => "JUMP_IF_FALSE".to_string(),
    opcode::OPCODE_LOAD_GLOBAL_SCOPE => "LOAD_GLOBAL_SCOPE".to_string(),
    opcode::OPCODE_SET_GLOBAL_SCOPE => "SET_GLOBAL_SCOPE".to_string(),
    opcode::OPCODE_LOAD_LOCAL_SCOPE => "LOAD_LOCAL_SCOPE".to_string(),
    opcode::OPCODE_SET_LOCAL_SCOPE => "SET_LOCAL_SCOPE".to_string(),
    opcode::OPCODE_POP => "POP".to_string(),
    _ => format!("Unknown opcode: {}", opcode),
  }
}

pub fn is_global_variable(name: &str) -> bool {
  match name {
    "globalThis " | "undefined " | "NaN " | "Infinity " | "Object " | "Function " | "Array " | "String "
    | "Number " | "Boolean " | "Math " | "Date " | "RegExp " | "Error " | "console " => true,
    _ => false,
  }
}
