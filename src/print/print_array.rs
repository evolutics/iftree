use super::print_initializer;
use crate::data;
use crate::model;

pub fn main(view: &model::View) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", data::ASSET_ARRAY_NAME);
    let type_ = &view.type_;
    let length = view.array.len();
    let expression = print_expression(view);

    quote::quote! { pub static #name: [#type_; #length] = #expression; }
}

fn print_expression(view: &model::View) -> proc_macro2::TokenStream {
    let content: proc_macro2::TokenStream = view
        .array
        .iter()
        .map(|file| {
            let element = print_initializer::main(view, file);
            quote::quote! { #element, }
        })
        .collect();

    quote::quote! { [#content] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_empty_set() {
        let actual = main(&model::View {
            type_: quote::format_ident!("Asset"),
            array: vec![],
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! { pub static ASSETS: [Asset; 0usize] = []; }.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_nonempty_set() {
        let actual = main(&model::View {
            type_: quote::format_ident!("Asset"),
            initializer: model::Initializer::Default(model::TypeStructure::TypeAlias(
                model::Template::RelativePath,
            )),
            array: vec![
                model::File {
                    relative_path: model::RelativePath::from("a"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("b/c"),
                    ..model::stubs::file()
                },
            ],
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ASSETS: [Asset; 2usize] = [
                "a",
                "b/c",
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
