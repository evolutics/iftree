use super::get_field_implementation;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    resource_structure: &model::Fields<()>,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    paths
        .into_iter()
        .map(|path| get_file(configuration, resource_structure, base_folder, path))
        .collect()
}

fn get_file(
    configuration: &model::Configuration,
    resource_structure: &model::Fields<()>,
    base_folder: &path::Path,
    relative_path: path::PathBuf,
) -> model::Result<model::File> {
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = absolute_path.to_string_lossy();

    let fields = match resource_structure {
        model::Fields::TypeAlias(()) => model::Fields::TypeAlias(get_field_implementation::main(
            configuration,
            absolute_path.as_ref(),
            model::FieldIdentifier::Anonymous,
        )?),

        model::Fields::NamedFields(fields) => model::Fields::NamedFields(
            fields
                .keys()
                .map(|name| {
                    let value = get_field_implementation::main(
                        configuration,
                        absolute_path.as_ref(),
                        model::FieldIdentifier::Named(name.clone()),
                    )?;
                    Ok((name.clone(), value))
                })
                .collect::<model::Result<_>>()?,
        ),

        model::Fields::TupleFields(_) => todo!(),
    };

    Ok(model::File {
        relative_path,
        fields,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_type_alias() {
        let actual = main(
            &model::Configuration {
                fields: vec![(
                    model::FieldIdentifier::Anonymous,
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            &model::Fields::TypeAlias(()),
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
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

    #[test]
    fn gets_named_fields() {
        let actual = main(
            &model::Configuration {
                fields: vec![(
                    model::FieldIdentifier::Named(String::from("content")),
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            &model::Fields::NamedFields(vec![(String::from("content"), ())].into_iter().collect()),
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                fields: model::Fields::NamedFields(
                    vec![(
                        String::from("content"),
                        quote::quote! {
                            include_str!("/resources/world/physical_constants.json")
                        },
                    )]
                    .into_iter()
                    .collect(),
                ),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                fields: model::Fields::NamedFields(
                    vec![(
                        String::from("content"),
                        quote::quote! {
                            include_str!("/resources/configuration/menu.json")
                        },
                    )]
                    .into_iter()
                    .collect(),
                ),
            },
        ];
        assert_eq!(actual, expected);
    }
}
