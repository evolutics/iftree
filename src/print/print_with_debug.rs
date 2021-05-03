use crate::data;
use crate::model;

pub fn main(view: model::View, code: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if view.debug {
        go(code)
    } else {
        code
    }
}

fn go(code: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", data::DEBUG_NAME);
    let value = code.to_string();

    quote::quote! {
        #code

        pub const #name: &str = #value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_no_debug() {
        let actual = main(
            model::View {
                debug: false,
                ..model::stubs::view()
            },
            quote::quote! { mod abc {} },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { mod abc {} }.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_debug() {
        let actual = main(
            model::View {
                debug: true,
                ..model::stubs::view()
            },
            quote::quote! { mod abc {} },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            mod abc {}

            pub const DEBUG: &str = "mod abc { }";
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
