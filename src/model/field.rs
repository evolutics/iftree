use super::main;
use crate::data;

impl From<String> for main::Field {
    fn from(string: String) -> Self {
        if string == data::ANONYMOUS_FIELD {
            main::Field::Anonymous
        } else {
            match string.parse() {
                Err(_) => main::Field::Named(string),
                Ok(index) => main::Field::Indexed(index),
            }
        }
    }
}

impl From<main::Field> for String {
    fn from(field: main::Field) -> Self {
        match field {
            main::Field::Anonymous => String::from(data::ANONYMOUS_FIELD),
            main::Field::Named(name) => name,
            main::Field::Indexed(index) => index.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod from_string {
        use super::*;

        #[test]
        fn handles_anonymous() {
            let actual = main::Field::from(String::from('_'));

            let expected = main::Field::Anonymous;
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_named() {
            let actual = main::Field::from(String::from("ab"));

            let expected = main::Field::Named(String::from("ab"));
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_indexed() {
            let actual = main::Field::from(String::from("12"));

            let expected = main::Field::Indexed(12);
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod into_string {
        use super::*;

        #[test]
        fn handles_anonymous() {
            let actual = String::from(main::Field::Anonymous);

            let expected = String::from('_');
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_named() {
            let actual = String::from(main::Field::Named(String::from("bc")));

            let expected = String::from("bc");
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_indexed() {
            let actual = String::from(main::Field::Indexed(23));

            let expected = String::from("23");
            assert_eq!(actual, expected);
        }
    }
}
