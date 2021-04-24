use crate::data;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    code: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if configuration.debug {
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
            &model::Configuration {
                debug: false,
                ..model::stubs::configuration()
            },
            quote::quote! { const CHEAT: &str = "abc"; },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { const CHEAT: &str = "abc"; }.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_debug() {
        let actual = main(
            &model::Configuration {
                debug: true,
                ..model::stubs::configuration()
            },
            quote::quote! { const CHEAT: &str = "abc"; },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            const CHEAT: &str = "abc";

            pub const DEBUG: &str = "const CHEAT : & str = \"abc\" ;";
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
