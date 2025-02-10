mod errors;
mod instructions;
mod parse;
mod vm;

pub use errors::Result;
pub use parse::parse_input;
pub use vm::VM;
