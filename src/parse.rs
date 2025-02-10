use crate::errors::Result;
use crate::instructions::Instruction;
use nom::{
    branch::alt,
    character::complete::{anychar, char},
    combinator::{map, value},
    multi::many0,
    IResult, Parser,
};

fn nom_instruction(input: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        value(Some(Instruction::MoveLeft), char('<')),
        value(Some(Instruction::MoveRight), char('>')),
        value(Some(Instruction::IncrementPtr), char('+')),
        value(Some(Instruction::DecrementPtr), char('-')),
        value(Some(Instruction::Output), char('.')),
        value(Some(Instruction::Input), char(',')),
        value(Some(Instruction::LoopStart), char('[')),
        value(Some(Instruction::LoopEnd), char(']')),
        value(None, anychar),
    ))
    .parse(input)
}

fn nom_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    map(
        many0(nom_instruction),
        |results: Vec<Option<Instruction>>| results.into_iter().flatten().collect(),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    let (_, instructions) = nom_instructions(input)?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_instruction() {
        assert_eq!(
            nom_instruction("+").unwrap(),
            ("", Some(Instruction::IncrementPtr))
        );

        assert_eq!(
            nom_instruction("-+").unwrap(),
            ("+", Some(Instruction::DecrementPtr))
        );

        assert_eq!(nom_instruction("foo").unwrap(), ("oo", None));
    }

    #[test]
    fn test_nom_instructions() {
        assert_eq!(
            nom_instructions("+foo-").unwrap(),
            (
                "",
                vec![Instruction::IncrementPtr, Instruction::DecrementPtr]
            )
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("+foo-").unwrap(),
            vec![Instruction::IncrementPtr, Instruction::DecrementPtr]
        );
    }
}
