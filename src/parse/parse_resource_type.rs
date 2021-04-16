use crate::model;
use syn::parse;

impl parse::Parse for model::ResourceType<()> {
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

fn parse_structure(item: parse::ParseStream) -> syn::Result<model::ResourceType<()>> {
    let derive_input = item.parse::<syn::DeriveInput>()?;

    let structure = match derive_input.data {
        syn::Data::Struct(data) => Ok(data),
        _ => Err(item.error("expected structure")),
    }?;

    let resource_structure = match structure.fields {
        syn::Fields::Unit => model::ResourceStructure::Unit,

        syn::Fields::Named(fields) => model::ResourceStructure::NamedFields(
            fields
                .named
                .into_iter()
                .filter_map(|field| field.ident.map(|identifier| (identifier.to_string(), ())))
                .collect(),
        ),

        syn::Fields::Unnamed(fields) => {
            model::ResourceStructure::TupleFields(fields.unnamed.iter().map(|_| ()).collect())
        }
    };

    Ok(model::ResourceType {
        identifier: derive_input.ident,
        structure: resource_structure,
    })
}

fn parse_type_alias(item: parse::ParseStream) -> syn::Result<model::ResourceType<()>> {
    item.parse::<syn::Token![type]>()?;
    let identifier = item.parse::<syn::Ident>()?;
    item.parse::<syn::Token![=]>()?;
    item.parse::<syn::Type>()?;
    item.parse::<syn::Token![;]>()?;

    Ok(model::ResourceType {
        identifier,
        structure: model::ResourceStructure::TypeAlias(()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_unit() {
        let actual = syn::parse_str::<model::ResourceType<()>>("pub struct MyUnit;");

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: quote::format_ident!("MyUnit"),
            structure: model::ResourceStructure::Unit,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_type_alias() {
        let actual =
            syn::parse_str::<model::ResourceType<()>>("pub type MyTypeAlias = &'static str;");

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: quote::format_ident!("MyTypeAlias"),
            structure: model::ResourceStructure::TypeAlias(()),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_named_fields() {
        let actual = syn::parse_str::<model::ResourceType<()>>(
            "pub struct MyNamedFields {
    content: &'static str,
    media_type: &'static str,
}",
        );

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: quote::format_ident!("MyNamedFields"),
            structure: model::ResourceStructure::NamedFields(vec![
                (String::from("content"), ()),
                (String::from("media_type"), ()),
            ]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_tuple_fields() {
        let actual = syn::parse_str::<model::ResourceType<()>>(
            "pub struct MyTupleFields(usize, &'static str);",
        );

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: quote::format_ident!("MyTupleFields"),
            structure: model::ResourceStructure::TupleFields(vec![(), ()]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_unexpected_item_it_errs() {
        let actual = syn::parse_str::<model::ResourceType<()>>("pub fn do_it() {}");

        let actual = actual.unwrap_err().to_string();
        assert_eq!(actual, "expected `struct` or `type`");
    }

    #[test]
    fn given_valid_but_unexpected_derive_input_it_errs() {
        let actual = syn::parse_str::<model::ResourceType<()>>(
            "pub union MyUnion {
    integer: u32,
    floating: f32,
}",
        );

        let actual = actual.is_err();
        assert!(actual);
    }
}
