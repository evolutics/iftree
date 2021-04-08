use super::render_field_template;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    templates: &model::AbstractResource<&model::Template>,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    Ok(paths
        .into_iter()
        .map(|path| get_file(templates, base_folder, path))
        .collect())
}

fn get_file(
    templates: &model::AbstractResource<&model::Template>,
    base_folder: &path::Path,
    relative_path: path::PathBuf,
) -> model::File {
    let raw_relative_path = &relative_path.to_string_lossy();
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = &absolute_path.to_string_lossy();
    let context = render_field_template::Context {
        relative_path: raw_relative_path,
        absolute_path,
    };

    let resource_term = match templates {
        model::AbstractResource::Unit => model::AbstractResource::Unit,

        model::AbstractResource::TypeAlias(template) => {
            model::AbstractResource::TypeAlias(render_field_template::main(template, &context))
        }

        model::AbstractResource::NamedFields(named_templates) => {
            model::AbstractResource::NamedFields(
                named_templates
                    .iter()
                    .map(|(name, template)| {
                        (
                            name.clone(),
                            render_field_template::main(template, &context),
                        )
                    })
                    .collect(),
            )
        }

        model::AbstractResource::TupleFields(templates) => model::AbstractResource::TupleFields(
            templates
                .iter()
                .map(|template| render_field_template::main(template, &context))
                .collect(),
        ),
    };

    model::File {
        relative_path,
        resource_term,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_type_unit() {
        let actual = main(
            &model::AbstractResource::Unit,
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
                resource_term: model::ResourceTerm::Unit,
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::Unit,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_type_alias() {
        let actual = main(
            &model::AbstractResource::TypeAlias(&model::Template::Content),
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
                resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                    include_str!("/resources/world/physical_constants.json")
                }),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                    include_str!("/resources/configuration/menu.json")
                }),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_type_named_fields() {
        let actual = main(
            &model::AbstractResource::NamedFields(vec![(
                String::from("content"),
                &model::Template::Content,
            )]),
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
                resource_term: model::ResourceTerm::NamedFields(vec![(
                    String::from("content"),
                    quote::quote! {
                        include_str!("/resources/world/physical_constants.json")
                    },
                )]),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::NamedFields(vec![(
                    String::from("content"),
                    quote::quote! {
                        include_str!("/resources/configuration/menu.json")
                    },
                )]),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_type_tuple_fields() {
        let actual = main(
            &model::AbstractResource::TupleFields(vec![&model::Template::Content]),
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
                resource_term: model::ResourceTerm::TupleFields(vec![quote::quote! {
                    include_str!("/resources/world/physical_constants.json")
                }]),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::TupleFields(vec![quote::quote! {
                    include_str!("/resources/configuration/menu.json")
                }]),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_template_context() {
        let actual = main(
            &model::AbstractResource::TupleFields(vec![
                &model::Template::RelativePath,
                &model::Template::AbsolutePath,
            ]),
            path::Path::new("/resources"),
            vec![path::PathBuf::from("credits.md")],
        );

        let actual = actual.unwrap();
        let expected = vec![model::File {
            relative_path: path::PathBuf::from("credits.md"),
            resource_term: model::ResourceTerm::TupleFields(vec![
                quote::quote! {
                    "credits.md"
                },
                quote::quote! {
                    "/resources/credits.md"
                },
            ]),
        }];
        assert_eq!(actual, expected);
    }
}
