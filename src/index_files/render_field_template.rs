use crate::model;
use std::cmp;

pub fn main<'a>(template: &str, context: &'a Context) -> model::Result<proc_macro2::TokenStream> {
    let absolute_path = context.absolute_path;

    match template {
        "include_str!({{absolute_path}})" => Ok(quote::quote! {
            include_str!(#absolute_path)
        }),

        _ => Err(model::Error::NonStandardTemplate(String::from(template))),
    }
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Context<'a> {
    pub absolute_path: &'a str,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn context<'a>() -> Context<'a> {
        Context {
            absolute_path: "/foo",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_include_str() {
        let actual = main(
            "include_str!({{absolute_path}})",
            &Context {
                absolute_path: "/credits.md",
            },
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_non_standard_template_it_errs() {
        let actual = main("my_include!({{absolute_path}})", &stubs::context());

        let actual = actual.unwrap_err();
        let expected =
            model::Error::NonStandardTemplate(String::from("my_include!({{absolute_path}})"));
        assert_eq!(actual, expected);
    }
}
