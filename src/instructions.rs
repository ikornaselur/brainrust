use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    IncrementPtr,
    DecrementPtr,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instruction = match self {
            Instruction::MoveRight => ">",
            Instruction::MoveLeft => "<",
            Instruction::IncrementPtr => "+",
            Instruction::DecrementPtr => "-",
            Instruction::Output => ".",
            Instruction::Input => ",",
            Instruction::LoopStart => "[",
            Instruction::LoopEnd => "]",
        };
        write!(f, "{}", instruction)
    }
}
