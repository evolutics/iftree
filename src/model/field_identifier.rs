use super::main;

const ANONYMOUS_IDENTIFIER: &str = "_";

impl From<&str> for main::FieldIdentifier {
    fn from(string: &str) -> Self {
        if string == ANONYMOUS_IDENTIFIER {
            main::FieldIdentifier::Anonymous
        } else {
            match string.parse() {
                Err(_) => main::FieldIdentifier::Named(String::from(string)),
                Ok(index) => main::FieldIdentifier::Indexed(index),
            }
        }
    }
}

impl From<main::FieldIdentifier> for String {
    fn from(identifier: main::FieldIdentifier) -> Self {
        match identifier {
            main::FieldIdentifier::Anonymous => String::from(ANONYMOUS_IDENTIFIER),
            main::FieldIdentifier::Named(name) => name,
            main::FieldIdentifier::Indexed(index) => index.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_string_into_anonymous() {
        let actual = main::FieldIdentifier::from("_");

        assert_eq!(actual, main::FieldIdentifier::Anonymous);
    }

    #[test]
    fn converts_string_into_named() {
        let actual = main::FieldIdentifier::from("foo");

        assert_eq!(actual, main::FieldIdentifier::Named(String::from("foo")));
    }

    #[test]
    fn converts_string_into_indexed() {
        let actual = main::FieldIdentifier::from("12");

        assert_eq!(actual, main::FieldIdentifier::Indexed(12));
    }

    #[test]
    fn converts_string_from_anonymous() {
        let actual = String::from(main::FieldIdentifier::Anonymous);

        assert_eq!(actual, String::from('_'));
    }

    #[test]
    fn converts_string_from_named() {
        let actual = String::from(main::FieldIdentifier::Named(String::from("bar")));

        assert_eq!(actual, String::from("bar"));
    }

    #[test]
    fn converts_string_from_indexed() {
        let actual = String::from(main::FieldIdentifier::Indexed(23));

        assert_eq!(actual, String::from("23"));
    }
}
