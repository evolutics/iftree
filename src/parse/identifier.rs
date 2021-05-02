use serde::de;
use std::cmp;
use std::fmt;

#[derive(cmp::PartialEq, Debug)]
pub struct Identifier(pub syn::Ident);

impl<'a> serde::Deserialize<'a> for Identifier {
    fn deserialize<T>(deserializer: T) -> Result<Identifier, T::Error>
    where
        T: serde::Deserializer<'a>,
    {
        deserializer.deserialize_str(IdentifierVisitor)
    }
}

struct IdentifierVisitor;

impl<'a> de::Visitor<'a> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an identifier")
    }

    fn visit_str<T>(self, string: &str) -> Result<Self::Value, T>
    where
        T: de::Error,
    {
        match syn::parse_str(string) {
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(string), &self)),
            Ok(identifier) => Ok(Identifier(identifier)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod deserialize {
        use super::*;

        #[derive(cmp::PartialEq, Debug, serde::Deserialize)]
        struct Binding {
            name: Identifier,
        }

        #[test]
        fn handles() {
            let actual: Result<Binding, _> = toml::from_str("name = 'my_value'");

            let actual = actual.unwrap();
            let expected = Binding {
                name: Identifier(quote::format_ident!("my_value")),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_invalid_identifier_it_errs() {
            let actual: Result<Binding, _> = toml::from_str("name = 'a bc'");

            let actual = actual.is_err();
            assert!(actual);
        }
    }
}
