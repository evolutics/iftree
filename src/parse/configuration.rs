use crate::model;
use std::fmt;
use syn::parse;
use toml::de;

impl parse::Parse for model::Configuration {
    fn parse(parameters: parse::ParseStream) -> syn::Result<Self> {
        let token = parameters.parse::<syn::LitStr>()?;
        toml::from_str(&token.value()).map_err(|error| refine_error(token, error))
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
    use std::path;

    #[test]
    fn parses_valid_configuration() {
        let actual = syn::parse_str::<model::Configuration>(
            r#""
            resource_folder = 'my/resources'
            ""#,
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            resource_folder: path::PathBuf::from("my/resources"),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_invalid_configuration() {
        let actual = syn::parse_str::<model::Configuration>(
            r#""
            resource_folder = #
            ""#,
        );

        let actual = actual.unwrap_err();
        let actual = format!("{}", actual);
        let expected = String::from(
            "expected a value, found a comment at line 2 column 31 (in the string) here:
            resource_folder = #
                              ▲",
        );
        assert_eq!(actual, expected);
    }
}
