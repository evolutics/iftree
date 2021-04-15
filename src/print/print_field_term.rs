use crate::model;
use std::cmp;

pub fn main(template: &model::Template, context: &Context) -> proc_macro2::TokenStream {
    let relative_path = context.relative_path;
    let absolute_path = context.absolute_path;

    match template {
        model::Template::Content => quote::quote! {
            include_str!(#absolute_path)
        },

        model::Template::GetContent => quote::quote! {{
            fn get() -> std::borrow::Cow<'static, str> {
                if cfg!(debug_assertions) {
                    std::borrow::Cow::from(std::fs::read_to_string(#absolute_path).unwrap())
                } else {
                    std::borrow::Cow::from(include_str!(#absolute_path))
                }
            }

            get
        }},

        model::Template::GetRawContent => quote::quote! {{
            fn get() -> std::borrow::Cow<'static, [u8]> {
                if cfg!(debug_assertions) {
                    std::borrow::Cow::from(std::fs::read(#absolute_path).unwrap())
                } else {
                    std::borrow::Cow::from(&include_bytes!(#absolute_path)[..])
                }
            }

            get
        }},

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
        fn prints_content() {
            let actual = main(
                &model::Template::Content,
                &Context {
                    absolute_path: "/a/b",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                include_str!("/a/b")
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn prints_get_content() {
            let actual = main(
                &model::Template::GetContent,
                &Context {
                    absolute_path: "/a/b",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {{
                fn get() -> std::borrow::Cow<'static, str> {
                    if cfg!(debug_assertions) {
                        std::borrow::Cow::from(std::fs::read_to_string("/a/b").unwrap())
                    } else {
                        std::borrow::Cow::from(include_str!("/a/b"))
                    }
                }

                get
            }}
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn prints_get_raw_content() {
            let actual = main(
                &model::Template::GetRawContent,
                &Context {
                    absolute_path: "/a/b",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {{
                fn get() -> std::borrow::Cow<'static, [u8]> {
                    if cfg!(debug_assertions) {
                        std::borrow::Cow::from(std::fs::read("/a/b").unwrap())
                    } else {
                        std::borrow::Cow::from(&include_bytes!("/a/b")[..])
                    }
                }

                get
            }}
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn prints_raw_content() {
            let actual = main(
                &model::Template::RawContent,
                &Context {
                    absolute_path: "/a/b",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                include_bytes!("/a/b")
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn prints_relative_path() {
            let actual = main(
                &model::Template::RelativePath,
                &Context {
                    relative_path: "a/b",
                    ..stubs::context()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                "a/b"
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }

    mod given_custom_template {
        use super::*;

        #[test]
        fn prints_macro() {
            let actual = main(
                &model::Template::Custom(String::from("my_include")),
                &Context {
                    relative_path: "b",
                    absolute_path: "/a/b",
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                my_include!("b", "/a/b")
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }
}
