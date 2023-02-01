use super::parse_configuration_from_string;
use crate::model;
use syn::parse;

impl parse::Parse for model::Configuration {
    fn parse(parameters: parse::ParseStream) -> syn::Result<Self> {
        let token = parameters.parse::<syn::LitStr>()?;
        parse_configuration_from_string::main(&token.value())
            .map_err(|error| syn::Error::new(token.span(), error))
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
            "TOML parse error at line 1, column 9
  |
1 | paths = #
  |         ^
invalid string
expected `\"`, `'`
",
        );
        assert_eq!(actual, expected);
    }
}
