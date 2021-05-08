use crate::model;
use syn::parse;

impl parse::Parse for model::Type<()> {
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

fn parse_structure(item: parse::ParseStream) -> syn::Result<model::Type<()>> {
    let derive_input = item.parse::<syn::DeriveInput>()?;

    let raw_structure = match derive_input.data {
        syn::Data::Struct(data) => Ok(data),
        _ => Err(item.error("expected structure")),
    }?;

    let structure = match raw_structure.fields {
        syn::Fields::Unit => model::TypeStructure::Unit,

        syn::Fields::Named(named_fields) => model::TypeStructure::NamedFields(
            named_fields
                .named
                .into_iter()
                .filter_map(|named_field| named_field.ident.map(|field| (field, ())))
                .collect(),
        ),

        syn::Fields::Unnamed(fields) => {
            model::TypeStructure::TupleFields(fields.unnamed.iter().map(|_| ()).collect())
        }
    };

    Ok(model::Type {
        name: derive_input.ident,
        structure,
    })
}

fn parse_type_alias(item: parse::ParseStream) -> syn::Result<model::Type<()>> {
    item.parse::<syn::Token![type]>()?;
    let name = item.parse::<syn::Ident>()?;
    item.parse::<syn::Token![=]>()?;
    item.parse::<syn::Type>()?;
    item.parse::<syn::Token![;]>()?;

    Ok(model::Type {
        name,
        structure: model::TypeStructure::TypeAlias(()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_unit() {
        let actual = syn::parse_str::<model::Type<()>>("pub struct MyUnit;");

        let actual = actual.unwrap();
        let expected = model::Type {
            name: quote::format_ident!("MyUnit"),
            structure: model::TypeStructure::Unit,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_type_alias() {
        let actual = syn::parse_str::<model::Type<()>>("pub type MyTypeAlias = &'static str;");

        let actual = actual.unwrap();
        let expected = model::Type {
            name: quote::format_ident!("MyTypeAlias"),
            structure: model::TypeStructure::TypeAlias(()),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_named_fields() {
        let actual = syn::parse_str::<model::Type<()>>(
            "pub struct MyNamedFields {
    ab: String,
    bc: &'static str,
}",
        );

        let actual = actual.unwrap();
        let expected = model::Type {
            name: quote::format_ident!("MyNamedFields"),
            structure: model::TypeStructure::NamedFields(vec![
                (quote::format_ident!("ab"), ()),
                (quote::format_ident!("bc"), ()),
            ]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_tuple_fields() {
        let actual =
            syn::parse_str::<model::Type<()>>("pub struct MyTupleFields(usize, &'static str);");

        let actual = actual.unwrap();
        let expected = model::Type {
            name: quote::format_ident!("MyTupleFields"),
            structure: model::TypeStructure::TupleFields(vec![(), ()]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_unexpected_item_it_errs() {
        let actual = syn::parse_str::<model::Type<()>>("pub fn do_it() {}");

        let actual = actual.unwrap_err().to_string();
        assert_eq!(actual, "expected `struct` or `type`");
    }

    #[test]
    fn given_valid_but_unexpected_derive_input_it_errs() {
        let actual = syn::parse_str::<model::Type<()>>(
            "pub union MyUnion {
    integer: u32,
    floating: f32,
}",
        );

        let actual = actual.is_err();
        assert!(actual);
    }
}
