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
