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
}
