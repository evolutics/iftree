use crate::model;
use std::cmp;

pub fn main(template: &model::Template, context: &Context) -> proc_macro2::TokenStream {
    let relative_path = context.relative_path;
    let absolute_path = context.absolute_path;

    match template {
        model::Template::AbsolutePath => quote::quote! {
            #absolute_path
        },
        model::Template::Content => quote::quote! {
            include_str!(#absolute_path)
        },
        model::Template::RawContent => quote::quote! {
            include_bytes!(#absolute_path)
        },
        model::Template::RelativePath => quote::quote! {
            #relative_path
        },

        model::Template::Custom(custom_macro) => {
            let macro_identifier = quote::format_ident!("{}", custom_macro);
            quote::quote! {
                #macro_identifier!(#relative_path, #absolute_path)
            }
        }
    }
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Context<'a> {
    pub relative_path: &'a str,
    pub absolute_path: &'a str,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn context<'a>() -> Context<'a> {
        Context {
            relative_path: "bar",
            absolute_path: "/foo/bar",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given_predefined_template {
        use super::*;

        #[test]
        fn renders_absolute_path() {
            let actual = main(
                &model::Template::AbsolutePath,
                &Context {
                    absolute_path: "/credits.md",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                "/credits.md"
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn renders_content() {
            let actual = main(
                &model::Template::Content,
                &Context {
                    absolute_path: "/credits.md",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                include_str!("/credits.md")
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn renders_raw_content() {
            let actual = main(
                &model::Template::RawContent,
                &Context {
                    absolute_path: "/credits.md",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                include_bytes!("/credits.md")
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn renders_relative_path() {
            let actual = main(
                &model::Template::RelativePath,
                &Context {
                    relative_path: "credits.md",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                "credits.md"
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }

    mod given_custom_template {
        use super::*;

        #[test]
        fn renders_macro() {
            let actual = main(
                &model::Template::Custom(String::from("my_include")),
                &Context {
                    relative_path: "credits.md",
                    absolute_path: "/resources/credits.md",
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                my_include!("credits.md", "/resources/credits.md")
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }
}
