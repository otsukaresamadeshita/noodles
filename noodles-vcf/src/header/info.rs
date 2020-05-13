mod key;
mod ty;

pub use self::ty::Type;

use std::{collections::HashMap, convert::TryFrom};

use super::{number, Number};

use self::key::Key;

#[derive(Clone, Debug)]
pub struct Info {
    id: String,
    number: Number,
    ty: Type,
    description: String,
    fields: HashMap<String, String>,
}

impl Info {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn number(&self) -> &Number {
        &self.number
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn fields(&self) -> &HashMap<String, String> {
        &self.fields
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingField(Key),
    InvalidNumber(number::ParseError),
    InvalidType(ty::ParseError),
}

impl TryFrom<&[(String, String)]> for Info {
    type Error = ParseError;

    fn try_from(fields: &[(String, String)]) -> Result<Self, Self::Error> {
        let mut it = fields.iter();

        let id = it
            .next()
            .ok_or_else(|| ParseError::MissingField(Key::Id))
            .and_then(|(k, v)| match k.parse() {
                Ok(Key::Id) => Ok(v.into()),
                _ => Err(ParseError::MissingField(Key::Id)),
            })?;

        let number = it
            .next()
            .ok_or_else(|| ParseError::MissingField(Key::Number))
            .and_then(|(k, v)| match k.parse() {
                Ok(Key::Number) => v.parse().map_err(ParseError::InvalidNumber),
                _ => Err(ParseError::MissingField(Key::Id)),
            })?;

        let ty = it
            .next()
            .ok_or_else(|| ParseError::MissingField(Key::Type))
            .and_then(|(k, v)| match k.parse() {
                Ok(Key::Type) => v.parse().map_err(ParseError::InvalidType),
                _ => Err(ParseError::MissingField(Key::Type)),
            })?;

        let description = it
            .next()
            .ok_or_else(|| ParseError::MissingField(Key::Description))
            .and_then(|(k, v)| match k.parse() {
                Ok(Key::Description) => Ok(v.into()),
                _ => Err(ParseError::MissingField(Key::Description)),
            })?;

        Ok(Self {
            id,
            number,
            ty,
            description,
            fields: it.cloned().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_fields() -> Vec<(String, String)> {
        vec![
            (String::from("ID"), String::from("NS")),
            (String::from("Number"), String::from("1")),
            (String::from("Type"), String::from("Integer")),
            (
                String::from("Description"),
                String::from("Number of samples with data"),
            ),
        ]
    }

    #[test]
    fn test_try_from_fields_for_info() -> Result<(), ParseError> {
        let fields = build_fields();
        let info = Info::try_from(&fields[..])?;

        assert_eq!(info.id(), "NS");
        assert_eq!(*info.number(), Number::Count(1));
        assert_eq!(*info.ty(), Type::Integer);
        assert_eq!(info.description(), "Number of samples with data");

        Ok(())
    }

    #[test]
    fn test_try_from_fields_for_info_with_extra_fields() -> Result<(), ParseError> {
        let mut fields = build_fields();
        fields.push((String::from("Source"), String::from("dbsnp")));
        fields.push((String::from("Version"), String::from("138")));

        let info = Info::try_from(&fields[..])?;

        let fields = info.fields();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields["Source"], "dbsnp");
        assert_eq!(fields["Version"], "138");

        Ok(())
    }
}