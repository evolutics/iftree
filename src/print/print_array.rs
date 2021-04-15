use super::print_resource_term;
use crate::data;
use crate::model;

pub fn main(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    let identifier = quote::format_ident!("{}", data::RESOURCE_ARRAY_IDENTIFIER);
    let type_identifier = &file_index.resource_type.identifier;
    let length = file_index.array.len();
    let expression = print_expression(file_index);

    quote::quote! {
        pub static #identifier: [#type_identifier; #length] = #expression;
    }
}

fn print_expression(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    let content: proc_macro2::TokenStream = file_index
        .array
        .iter()
        .map(|file| {
            let element = print_resource_term::main(&file_index.resource_type, file);
            quote::quote! { #element, }
        })
        .collect();

    quote::quote! {
        [#content]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_empty_set() {
        let actual = main(&model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                ..model::stubs::resource_type()
            },
            array: vec![],
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ARRAY: [Resource; 0usize] = [];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_files() {
        let actual = main(&model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TypeAlias(model::Template::RelativePath),
            },
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
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ARRAY: [Resource; 2usize] = [
                "a",
                "b/c",
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
