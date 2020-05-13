use std::{error, fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Character,
    String,
}

#[derive(Debug)]
pub struct ParseError(String);

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid format type: expected {{Integer, Float, Character, String}}, got {}",
            self.0
        )
    }
}

impl FromStr for Type {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Integer" => Ok(Self::Integer),
            "Float" => Ok(Self::Float),
            "Character" => Ok(Self::Character),
            "String" => Ok(Self::String),
            _ => Err(ParseError(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        assert_eq!("Integer".parse::<Type>()?, Type::Integer);
        assert_eq!("Float".parse::<Type>()?, Type::Float);
        assert_eq!("Character".parse::<Type>()?, Type::Character);
        assert_eq!("String".parse::<Type>()?, Type::String);

        assert!("".parse::<Type>().is_err());
        assert!("Noodles".parse::<Type>().is_err());

        Ok(())
    }
}