use crate::model;
use std::cmp;

pub fn main(template: &model::Template, context: &Context) -> proc_macro2::TokenStream {
    let relative_path = context.relative_path;
    let absolute_path = context.absolute_path;

    match template {
        model::Template::ContentsBytes => quote::quote! { include_bytes!(#absolute_path) },

        model::Template::ContentsStr => quote::quote! { include_str!(#absolute_path) },

        model::Template::GetBytes => quote::quote! {{
            fn get() -> std::borrow::Cow<'static, [u8]> {
                if cfg!(debug_assertions) {
                    std::borrow::Cow::from(std::fs::read(#absolute_path).unwrap())
                } else {
                    std::borrow::Cow::from(&include_bytes!(#absolute_path)[..])
                }
            }

            get
        }},

        model::Template::GetStr => quote::quote! {{
            fn get() -> std::borrow::Cow<'static, str> {
                if cfg!(debug_assertions) {
                    std::borrow::Cow::from(std::fs::read_to_string(#absolute_path).unwrap())
                } else {
                    std::borrow::Cow::from(include_str!(#absolute_path))
                }
            }

            get
        }},

        model::Template::RelativePath => quote::quote! { #relative_path },
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

    #[test]
    fn handles_contents_bytes() {
        let actual = main(
            &model::Template::ContentsBytes,
            &Context {
                absolute_path: "/a/b",
                ..stubs::context()
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { include_bytes!("/a/b") }.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_contents_str() {
        let actual = main(
            &model::Template::ContentsStr,
            &Context {
                absolute_path: "/a/b",
                ..stubs::context()
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { include_str!("/a/b") }.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_get_bytes() {
        let actual = main(
            &model::Template::GetBytes,
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
    fn handles_get_str() {
        let actual = main(
            &model::Template::GetStr,
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
    fn handles_relative_path() {
        let actual = main(
            &model::Template::RelativePath,
            &Context {
                relative_path: "a/b",
                ..stubs::context()
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { "a/b" }.to_string();
        assert_eq!(actual, expected);
    }
}
