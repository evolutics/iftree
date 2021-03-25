use crate::model;
use syn::parse;

impl parse::Parse for model::TypeAlias {
    fn parse(item: parse::ParseStream) -> syn::Result<Self> {
        item.call(syn::Attribute::parse_outer)?;
        item.parse::<syn::Visibility>()?;
        item.parse::<syn::Token![type]>()?;
        let identifier = item.parse::<syn::Ident>()?;
        item.parse::<syn::Token![=]>()?;
        item.parse::<syn::Type>()?;
        item.parse::<syn::Token![;]>()?;

        Ok(model::TypeAlias {
            identifier: identifier.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_type_alias() {
        let actual = syn::parse_str::<model::TypeAlias>("pub type MyResource = &'static str;");

        let actual = actual.unwrap();
        let expected = model::TypeAlias {
            identifier: String::from("MyResource"),
        };
        assert_eq!(actual, expected);
    }
}
