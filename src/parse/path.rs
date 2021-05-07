use super::configuration;
use serde::de;
use std::fmt;

impl<'a> serde::Deserialize<'a> for configuration::Path {
    fn deserialize<T>(deserializer: T) -> Result<configuration::Path, T::Error>
    where
        T: serde::Deserializer<'a>,
    {
        deserializer.deserialize_str(Visitor)
    }
}

struct Visitor;

impl<'a> de::Visitor<'a> for Visitor {
    type Value = configuration::Path;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a path (like `x` or `x::y::z`)")
    }

    fn visit_str<T>(self, string: &str) -> Result<Self::Value, T>
    where
        T: de::Error,
    {
        match syn::parse_str(string) {
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(string), &self)),
            Ok(value) => Ok(configuration::Path(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod deserialize {
        use super::*;
        use std::cmp;

        #[derive(cmp::PartialEq, Debug, serde::Deserialize)]
        struct Binding {
            name: configuration::Path,
        }

        #[test]
        fn handles_one_segment() {
            let actual: Result<Binding, _> = toml::from_str("name = 'my_value'");

            let actual = actual.unwrap();
            let expected = Binding {
                name: configuration::Path(syn::parse_str("my_value").unwrap()),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_multiple_segments() {
            let actual: Result<Binding, _> = toml::from_str("name = 'a::b::c'");

            let actual = actual.unwrap();
            let expected = Binding {
                name: configuration::Path(syn::parse_str("a::b::c").unwrap()),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_invalid_value_it_errs() {
            let actual: Result<Binding, _> = toml::from_str("name = 'a bc'");

            let actual = actual.is_err();
            assert!(actual);
        }
    }
}
