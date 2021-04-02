use crate::model;
use std::path;
use std::vec;

pub fn main(base_folder: &path::Path, paths: vec::Vec<path::PathBuf>) -> vec::Vec<model::File> {
    paths
        .into_iter()
        .map(|path| get_file(base_folder, path))
        .collect()
}

fn get_file(base_folder: &path::Path, relative_path: path::PathBuf) -> model::File {
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = absolute_path.to_string_lossy();

    model::File {
        relative_path,
        fields: model::Fields::TypeAlias(quote::quote! {
            include_str!(#absolute_path)
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main(
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                fields: model::Fields::TypeAlias(quote::quote! {
                    include_str!("/resources/world/physical_constants.json")
                }),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                fields: model::Fields::TypeAlias(quote::quote! {
                    include_str!("/resources/configuration/menu.json")
                }),
            },
        ];
        assert_eq!(actual, expected);
    }
}
