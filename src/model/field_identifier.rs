use super::main;

const ANONYMOUS_IDENTIFIER: &str = "_";

impl From<String> for main::FieldIdentifier {
    fn from(string: String) -> Self {
        if string == ANONYMOUS_IDENTIFIER {
            main::FieldIdentifier::Anonymous
        } else {
            match string.parse() {
                Err(_) => main::FieldIdentifier::Named(string),
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
        let actual = main::FieldIdentifier::from(String::from('_'));

        let expected = main::FieldIdentifier::Anonymous;
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_string_into_named() {
        let actual = main::FieldIdentifier::from(String::from("foo"));

        let expected = main::FieldIdentifier::Named(String::from("foo"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_string_into_indexed() {
        let actual = main::FieldIdentifier::from(String::from("12"));

        let expected = main::FieldIdentifier::Indexed(12);
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_string_from_anonymous() {
        let actual = String::from(main::FieldIdentifier::Anonymous);

        let expected = String::from('_');
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_string_from_named() {
        let actual = String::from(main::FieldIdentifier::Named(String::from("bar")));

        let expected = String::from("bar");
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_string_from_indexed() {
        let actual = String::from(main::FieldIdentifier::Indexed(23));

        let expected = String::from("23");
        assert_eq!(actual, expected);
    }
}
