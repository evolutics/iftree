use super::print_forest;
use super::print_with_debug;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    item: proc_macro2::TokenStream,
    view: model::View,
) -> proc_macro2::TokenStream {
    let visits = view
        .visitors
        .iter()
        .map(|visitor| print_forest::main(&view, visitor))
        .collect::<proc_macro2::TokenStream>();

    let code = quote::quote! {
        #item

        #visits
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
                visitors: vec![
                    model::Visitor::Array(model::Initializer::Default(
                        model::TypeStructure::TypeAlias(model::Populator::ContentsStr),
                    )),
                    model::Visitor::Identifiers,
                ],
                forest: vec![(
                    String::new(),
                    model::Tree::File(model::File {
                        identifier: quote::format_ident!("A_B"),
                        index: 0,
                        absolute_path: String::from("/a.b"),
                        ..model::stubs::file()
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
