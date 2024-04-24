//! The Engine bytecode.

#[allow(dead_code)]
/// Opcodes for Engine bytecode instructions.
pub const OPCODE_HALF: u8 = 0x00; // stop the program
pub const OPCODE_CONST: u8 = 0x01; // push a constant onto the stack
pub const OPCODE_ADD: u8 = 0x02; // add two values
pub const OPCODE_SUB: u8 = 0x03; // subtract two values
pub const OPCODE_MUL: u8 = 0x04; // multiply two values
pub const OPCODE_DIV: u8 = 0x05; // divide two values
pub const OPCODE_MOD: u8 = 0x06; // modulo two values
pub const OPCODE_NEG: u8 = 0x07; // negate a value
pub const OPCODE_NOT: u8 = 0x08; // logical not
pub const OPCODE_AND: u8 = 0x09; // logical and
pub const OPCODE_OR: u8 = 0x0A; // logical or
pub const OPCODE_XOR: u8 = 0x0B; // logical xor
pub const OPCODE_SHL: u8 = 0x0C; // shift left
pub const OPCODE_SHR: u8 = 0x0D; // shift right
pub const OPCODE_LT: u8 = 0x0E; // less than
pub const OPCODE_LE: u8 = 0x0F; // less than or equal
pub const OPCODE_GT: u8 = 0x10; // greater than
pub const OPCODE_GE: u8 = 0x11; // greater than or equal
pub const OPCODE_EQ: u8 = 0x12; // equal
pub const OPCODE_NE: u8 = 0x13; // not equal
