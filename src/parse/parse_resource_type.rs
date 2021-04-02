use crate::model;
use syn::parse;

impl parse::Parse for model::ResourceType {
    fn parse(item: parse::ParseStream) -> syn::Result<Self> {
        item.call(syn::Attribute::parse_outer)?;
        item.parse::<syn::Visibility>()?;
        item.parse::<syn::Token![type]>()?;
        let identifier = item.parse::<syn::Ident>()?;
        item.parse::<syn::Token![=]>()?;
        item.parse::<syn::Type>()?;
        item.parse::<syn::Token![;]>()?;

        Ok(model::ResourceType {
            identifier: identifier.to_string(),
            structure: model::Fields::TypeAlias(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_type_alias() {
        let actual = syn::parse_str::<model::ResourceType>("pub type MyResource = &'static str;");

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: String::from("MyResource"),
            structure: model::Fields::TypeAlias(()),
        };
        assert_eq!(actual, expected);
    }
}
