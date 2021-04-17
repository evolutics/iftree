use super::parse_configuration_from_string;
use crate::model;
use std::fmt;
use syn::parse;
use toml::de;

impl parse::Parse for model::Configuration {
    fn parse(parameters: parse::ParseStream) -> syn::Result<Self> {
        let token = parameters.parse::<syn::LitStr>()?;
        parse_configuration_from_string::main(&token.value())
            .map_err(|error| refine_error(token, error))
    }
}

fn refine_error(token: syn::LitStr, error: de::Error) -> syn::Error {
    syn::Error::new(token.span(), InStringError { token, error })
}

struct InStringError {
    token: syn::LitStr,
    error: de::Error,
}

impl fmt::Display for InStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let appendix = match self.error.line_col() {
            None => String::new(),
            Some((line_index, column_index)) => match self.token.value().lines().nth(line_index) {
                None => String::new(),
                Some(line) => {
                    let space = " ".repeat(column_index);
                    format!(" here:\n{}\n{}▲", line, space)
                }
            },
        };

        write!(formatter, "{} (in the string){}", self.error, appendix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_valid_configuration() {
        let actual = syn::parse_str::<model::Configuration>(r#""paths = '/assets'""#);

        let actual = actual.is_ok();
        assert!(actual);
    }

    #[test]
    fn given_invalid_configuration_it_errs() {
        let actual = syn::parse_str::<model::Configuration>(r#""paths = #""#);

        let actual = actual.unwrap_err().to_string();
        let expected = String::from(
            "expected a value, found a comment at line 1 column 9 (in the string) here:
paths = #
        ▲",
        );
        assert_eq!(actual, expected);
    }
}
