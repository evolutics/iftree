use crate::model;

pub fn main(fields: &model::Fields<proc_macro2::TokenStream>) -> &proc_macro2::TokenStream {
    match fields {
        model::Fields::TypeAlias(field) => field,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints() {
        let fields = model::Fields::TypeAlias(quote::quote! {
            include_str!("/credits.md")
        });

        let actual = main(&fields);

        let actual = actual.to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
