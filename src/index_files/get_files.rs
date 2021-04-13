use super::render_field_template;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    templates: &model::ResourceStructure<&model::Template>,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    paths
        .into_iter()
        .map(|path| get_file(templates, base_folder, path))
        .collect()
}

fn get_file(
    templates: &model::ResourceStructure<&model::Template>,
    base_folder: &path::Path,
    absolute_path: path::PathBuf,
) -> model::Result<model::File> {
    let relative_path = model::RelativePath(String::from(
        absolute_path.strip_prefix(base_folder)?.to_string_lossy(),
    ));
    let context = render_field_template::Context {
        relative_path: &relative_path.0,
        absolute_path: &absolute_path.to_string_lossy(),
    };

    let resource_term = match templates {
        model::ResourceStructure::Unit => model::ResourceStructure::Unit,

        model::ResourceStructure::TypeAlias(template) => {
            model::ResourceStructure::TypeAlias(render_field_template::main(template, &context))
        }

        model::ResourceStructure::NamedFields(named_templates) => {
            model::ResourceStructure::NamedFields(
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

        model::ResourceStructure::TupleFields(templates) => model::ResourceStructure::TupleFields(
            templates
                .iter()
                .map(|template| render_field_template::main(template, &context))
                .collect(),
        ),
    };

    Ok(model::File {
        relative_path,
        resource_term,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_template_context() {
        let actual = main(
            &model::ResourceStructure::TupleFields(vec![
                &model::Template::RelativePath,
                &model::Template::Content,
            ]),
            path::Path::new("/resources"),
            vec![path::PathBuf::from("/resources/credits.md")],
        );

        let actual = actual.unwrap();
        let expected = vec![model::File {
            relative_path: model::RelativePath::from("credits.md"),
            resource_term: model::ResourceTerm::TupleFields(vec![
                quote::quote! {
                    "credits.md"
                },
                quote::quote! {
                    include_str!("/resources/credits.md")
                },
            ]),
        }];
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod resource_cases {
        use super::*;

        #[test]
        fn gets_unit() {
            let actual = main(
                &model::ResourceStructure::Unit,
                path::Path::new("/resources"),
                vec![
                    path::PathBuf::from("/resources/world/physical_constants.json"),
                    path::PathBuf::from("/resources/configuration/menu.json"),
                ],
            );

            let actual = actual.unwrap();
            let expected = vec![
                model::File {
                    relative_path: model::RelativePath::from("world/physical_constants.json"),
                    resource_term: model::ResourceTerm::Unit,
                },
                model::File {
                    relative_path: model::RelativePath::from("configuration/menu.json"),
                    resource_term: model::ResourceTerm::Unit,
                },
            ];
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_type_alias() {
            let actual = main(
                &model::ResourceStructure::TypeAlias(&model::Template::Content),
                path::Path::new("/resources"),
                vec![
                    path::PathBuf::from("/resources/world/physical_constants.json"),
                    path::PathBuf::from("/resources/configuration/menu.json"),
                ],
            );

            let actual = actual.unwrap();
            let expected = vec![
                model::File {
                    relative_path: model::RelativePath::from("world/physical_constants.json"),
                    resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                        include_str!("/resources/world/physical_constants.json")
                    }),
                },
                model::File {
                    relative_path: model::RelativePath::from("configuration/menu.json"),
                    resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                        include_str!("/resources/configuration/menu.json")
                    }),
                },
            ];
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_named_fields() {
            let actual = main(
                &model::ResourceStructure::NamedFields(vec![(
                    String::from("content"),
                    &model::Template::Content,
                )]),
                path::Path::new("/resources"),
                vec![
                    path::PathBuf::from("/resources/world/physical_constants.json"),
                    path::PathBuf::from("/resources/configuration/menu.json"),
                ],
            );

            let actual = actual.unwrap();
            let expected = vec![
                model::File {
                    relative_path: model::RelativePath::from("world/physical_constants.json"),
                    resource_term: model::ResourceTerm::NamedFields(vec![(
                        String::from("content"),
                        quote::quote! {
                            include_str!("/resources/world/physical_constants.json")
                        },
                    )]),
                },
                model::File {
                    relative_path: model::RelativePath::from("configuration/menu.json"),
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
        fn gets_tuple_fields() {
            let actual = main(
                &model::ResourceStructure::TupleFields(vec![&model::Template::Content]),
                path::Path::new("/resources"),
                vec![
                    path::PathBuf::from("/resources/world/physical_constants.json"),
                    path::PathBuf::from("/resources/configuration/menu.json"),
                ],
            );

            let actual = actual.unwrap();
            let expected = vec![
                model::File {
                    relative_path: model::RelativePath::from("world/physical_constants.json"),
                    resource_term: model::ResourceTerm::TupleFields(vec![quote::quote! {
                        include_str!("/resources/world/physical_constants.json")
                    }]),
                },
                model::File {
                    relative_path: model::RelativePath::from("configuration/menu.json"),
                    resource_term: model::ResourceTerm::TupleFields(vec![quote::quote! {
                        include_str!("/resources/configuration/menu.json")
                    }]),
                },
            ];
            assert_eq!(actual, expected);
        }
    }
}
