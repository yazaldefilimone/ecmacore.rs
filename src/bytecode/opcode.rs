//! The Engine bytecode.
#![allow(dead_code)]
/// Opcodes for Engine bytecode instructions.
pub const OPCODE_HALF: usize = 0x00; // stop the program
pub const OPCODE_CONST: usize = 0x01; // push a constant onto the stack
pub const OPCODE_ADD: usize = 0x02; // add two values
pub const OPCODE_SUB: usize = 0x03; // subtract two values
pub const OPCODE_MUL: usize = 0x04; // multiply two values
pub const OPCODE_DIV: usize = 0x05; // divide two values
pub const OPCODE_MOD: usize = 0x06; // modulo two values
pub const OPCODE_NEG: usize = 0x07; // negate a value
pub const OPCODE_NOT: usize = 0x08; // logical not
pub const OPCODE_AND: usize = 0x09; // logical and
pub const OPCODE_OR: usize = 0x0A; // logical or
pub const OPCODE_XOR: usize = 0x0B; // logical xor
pub const OPCODE_SHL: usize = 0x0C; // shift left
pub const OPCODE_SHR: usize = 0x0D; // shift right
pub const OPCODE_LT: usize = 0x0E; // less than
pub const OPCODE_LE: usize = 0x0F; // less than or equal
pub const OPCODE_GT: usize = 0x10; // greater than
pub const OPCODE_GE: usize = 0x11; // greater than or equal
pub const OPCODE_EQ: usize = 0x12; // equal
pub const OPCODE_NE: usize = 0x13; // not equal
pub const OPCODE_JUMP: usize = 0x14; // jump to an instruction
pub const OPCODE_JUMP_IF_FALSE: usize = 0x15; // jump to an instruction if a value is true
                                              // variables
pub const OPCODE_LOAD_GLOBAL_SCOPE: usize = 0x17; // load a global variable
pub const OPCODE_SET_GLOBAL_SCOPE: usize = 0x18; // store a global variable
pub const OPCODE_LOAD_LOCAL_SCOPE: usize = 0x19; // load a local variable
pub const OPCODE_SET_LOCAL_SCOPE: usize = 0x1A; // store a local variable

//
pub const OPCODE_POP: usize = 0x16; // pop a value from the stack
pub const OPCODE_SCOPE_EXIT: usize = 0x17;
