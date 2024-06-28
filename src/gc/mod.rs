/*

Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>

*/

mod gc_value;
mod heap;
mod stack_frame;

pub use self::gc_value::GCValue;
pub use self::heap::Heap;
pub use self::stack_frame::StackFrame;
