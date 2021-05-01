use super::print_array;
use super::print_identifiers;
use super::print_with_debug;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    item: proc_macro2::TokenStream,
    view: model::View,
) -> proc_macro2::TokenStream {
    let array = print_array::main(&view);
    let identifiers = print_identifiers::main(&view);

    let code = quote::quote! {
        #item

        #array

        #identifiers
    };

    print_with_debug::main(configuration, code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(
            &model::Configuration {
                debug: false,
                ..model::stubs::configuration()
            },
            quote::quote! { pub type Asset = &'static str; },
            model::View {
                type_: quote::format_ident!("Asset"),
                initializer: model::Initializer::Default(model::TypeStructure::TypeAlias(
                    model::Populator::ContentsStr,
                )),
                array: vec![model::Path {
                    absolute: String::from("/a.b"),
                    ..model::stubs::path()
                }],
                forest: vec![(
                    String::new(),
                    model::FileTree::Folder(model::Folder {
                        identifier: quote::format_ident!("base"),
                        forest: vec![(
                            String::new(),
                            model::FileTree::File(model::File {
                                identifier: quote::format_ident!("A_B"),
                                index: 0,
                            }),
                        )]
                        .into_iter()
                        .collect(),
                    }),
                )]
                .into_iter()
                .collect(),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub type Asset = &'static str;

            pub static ASSETS: [Asset; 1usize] = [
                include_str!("/a.b"),
            ];

            pub mod base {
                pub static A_B: &super::Asset = &super::ASSETS[0usize];
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
