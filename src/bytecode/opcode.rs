//! The Engine bytecode.
#![allow(dead_code)]

/// Opcodes for Engine bytecode instructions.
pub const OPCODE_HALF: usize = 0x00; // Stop the program
pub const OPCODE_CONST: usize = 0x01; // Push a constant onto the stack
pub const OPCODE_ADD: usize = 0x02; // Add two values
pub const OPCODE_SUB: usize = 0x03; // Subtract two values
pub const OPCODE_MUL: usize = 0x04; // Multiply two values
pub const OPCODE_DIV: usize = 0x05; // Divide two values
pub const OPCODE_MOD: usize = 0x06; // Modulo two values
pub const OPCODE_NEG: usize = 0x07; // Negate a value
pub const OPCODE_NOT: usize = 0x08; // Logical not
pub const OPCODE_AND: usize = 0x09; // Logical and
pub const OPCODE_OR: usize = 0x0A; // Logical or
pub const OPCODE_XOR: usize = 0x0B; // Logical xor
pub const OPCODE_SHL: usize = 0x0C; // Shift left
pub const OPCODE_SHR: usize = 0x0D; // Shift right
pub const OPCODE_LT: usize = 0x0E; // Less than
pub const OPCODE_LE: usize = 0x0F; // Less than or equal
pub const OPCODE_GT: usize = 0x10; // Greater than
pub const OPCODE_GE: usize = 0x11; // Greater than or equal
pub const OPCODE_EQ: usize = 0x12; // Equal (===)
pub const OPCODE_WEAK_EQ: usize = 0x13; // Weak equal (==)
pub const OPCODE_NE: usize = 0x14; // Not equal (!=)
pub const OPCODE_JUMP: usize = 0x15; // Jump to an instruction
pub const OPCODE_JUMP_IF_FALSE: usize = 0x16; // Jump to an instruction if a value is false
pub const OPCODE_LOAD_GLOBAL_SCOPE: usize = 0x17; // Load a global variable
pub const OPCODE_SET_GLOBAL_SCOPE: usize = 0x18; // Store a global variable
pub const OPCODE_LOAD_LOCAL_SCOPE: usize = 0x19; // Load a local variable
pub const OPCODE_SET_LOCAL_SCOPE: usize = 0x1A; // Store a local variable
pub const OPCODE_POP: usize = 0x1B; // Pop a value from the stack
pub const OPCODE_SCOPE_EXIT: usize = 0x1C; // Exit the current scope
