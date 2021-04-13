use crate::model;
use syn::parse;

impl parse::Parse for model::ResourceType {
    fn parse(item: parse::ParseStream) -> syn::Result<Self> {
        item.call(syn::Attribute::parse_outer)?;
        item.parse::<syn::Visibility>()?;

        let lookahead = item.lookahead1();
        if lookahead.peek(syn::Token![struct]) {
            parse_structure(item)
        } else if lookahead.peek(syn::Token![type]) {
            parse_type_alias(item)
        } else {
            Err(lookahead.error())
        }
    }
}

fn parse_structure(item: parse::ParseStream) -> syn::Result<model::ResourceType> {
    let derive_input = item.parse::<syn::DeriveInput>()?;

    let structure = match derive_input.data {
        syn::Data::Struct(data) => Ok(data),
        _ => Err(item.error("expected structure")),
    }?;

    let resource_structure = match structure.fields {
        syn::Fields::Unit => model::AbstractResource::Unit,

        syn::Fields::Named(fields) => model::AbstractResource::NamedFields(
            fields
                .named
                .into_iter()
                .filter_map(|field| field.ident.map(|identifier| (identifier.to_string(), ())))
                .collect(),
        ),

        syn::Fields::Unnamed(fields) => {
            model::AbstractResource::TupleFields(fields.unnamed.iter().map(|_| ()).collect())
        }
    };

    Ok(model::ResourceType {
        identifier: derive_input.ident.to_string(),
        structure: resource_structure,
    })
}

fn parse_type_alias(item: parse::ParseStream) -> syn::Result<model::ResourceType> {
    item.parse::<syn::Token![type]>()?;
    let identifier = item.parse::<syn::Ident>()?;
    item.parse::<syn::Token![=]>()?;
    item.parse::<syn::Type>()?;
    item.parse::<syn::Token![;]>()?;

    Ok(model::ResourceType {
        identifier: identifier.to_string(),
        structure: model::AbstractResource::TypeAlias(()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unit() {
        let actual = syn::parse_str::<model::ResourceType>("pub struct MyUnit;");

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: String::from("MyUnit"),
            structure: model::AbstractResource::Unit,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_type_alias() {
        let actual = syn::parse_str::<model::ResourceType>("pub type MyTypeAlias = &'static str;");

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: String::from("MyTypeAlias"),
            structure: model::AbstractResource::TypeAlias(()),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_named_fields() {
        let actual = syn::parse_str::<model::ResourceType>(
            "pub struct MyNamedFields {
    content: &'static str,
    media_type: &'static str,
}",
        );

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: String::from("MyNamedFields"),
            structure: model::AbstractResource::NamedFields(vec![
                (String::from("content"), ()),
                (String::from("media_type"), ()),
            ]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_tuple_fields() {
        let actual =
            syn::parse_str::<model::ResourceType>("pub struct MyTupleFields(usize, &'static str);");

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: String::from("MyTupleFields"),
            structure: model::AbstractResource::TupleFields(vec![(), ()]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_unexpected_item_it_errs() {
        let actual = syn::parse_str::<model::ResourceType>("pub fn do_it() {}");

        let actual = actual.unwrap_err().to_string();
        assert_eq!(actual, "expected `struct` or `type`");
    }

    #[test]
    fn given_valid_but_unexpected_derive_input_it_errs() {
        let actual = syn::parse_str::<model::ResourceType>(
            "pub union MyUnion {
    integer: u32,
    floating: f32,
}",
        );

        let actual = actual.is_err();
        assert!(actual);
    }
}
