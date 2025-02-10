use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    IncrementPtr,
    DecrementPtr,
    OutputChar,
    InputChar,
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
            Instruction::OutputChar => ",",
            Instruction::InputChar => ",",
            Instruction::LoopStart => "[",
            Instruction::LoopEnd => "]",
        };
        write!(f, "{}", instruction)
    }
}
