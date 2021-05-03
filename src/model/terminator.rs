use super::main;
use quote::TokenStreamExt;

impl quote::ToTokens for main::Terminator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            main::Terminator::Void => {}
            main::Terminator::Comma => {
                tokens.append(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod to_tokens {
        use super::*;

        #[test]
        fn handles_void() {
            let value = main::Terminator::Void;

            let actual = quote::quote! { #value };

            let actual = actual.to_string();
            let expected = quote::quote! {}.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_comma() {
            let value = main::Terminator::Comma;

            let actual = quote::quote! { #value };

            let actual = actual.to_string();
            let expected = quote::quote! { , }.to_string();
            assert_eq!(actual, expected);
        }
    }
}
