use crate::model;
use std::collections;

pub fn main(
    structure: model::TypeStructure<()>,
) -> model::Result<model::TypeStructure<model::Populator>> {
    match structure {
        model::TypeStructure::Unit => Ok(model::TypeStructure::Unit),

        model::TypeStructure::TypeAlias(_) => Err(model::Error::NoInitializer),

        model::TypeStructure::NamedFields(fields) => {
            let standard_field_populators = get_standard_field_populators();
            Ok(model::TypeStructure::NamedFields(
                fields
                    .into_iter()
                    .map(|(field, _)| match standard_field_populators.get(&field) {
                        None => Err(model::Error::NonstandardField {
                            field: field.clone(),
                            standard_fields: standard_field_populators.keys().cloned().collect(),
                        }),
                        Some(populator) => Ok((field, populator.clone())),
                    })
                    .collect::<model::Result<_>>()?,
            ))
        }

        model::TypeStructure::TupleFields(unary_length) => {
            if unary_length.is_empty() {
                Ok(model::TypeStructure::TupleFields(vec![]))
            } else {
                Err(model::Error::NoInitializer)
            }
        }
    }
}

fn get_standard_field_populators() -> collections::BTreeMap<syn::Ident, model::Populator> {
    vec![
        (
            quote::format_ident!("contents_bytes"),
            model::Populator::ContentsBytes,
        ),
        (
            quote::format_ident!("contents_str"),
            model::Populator::ContentsStr,
        ),
        (
            quote::format_ident!("get_bytes"),
            model::Populator::GetBytes,
        ),
        (quote::format_ident!("get_str"), model::Populator::GetStr),
        (
            quote::format_ident!("relative_path"),
            model::Populator::RelativePath,
        ),
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_unit() {
        let actual = main(model::TypeStructure::Unit);

        let actual = actual.unwrap();
        let expected = model::TypeStructure::Unit;
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_type_alias() {
        let actual = main(model::TypeStructure::TypeAlias(()));

        let actual = actual.unwrap_err();
        let expected = model::Error::NoInitializer;
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod handles_named_fields {
        use super::*;

        #[test]
        fn given_standard_fields_only_it_handles() {
            let actual = main(model::TypeStructure::NamedFields(vec![
                (quote::format_ident!("relative_path"), ()),
                (quote::format_ident!("contents_str"), ()),
            ]));

            let actual = actual.unwrap();
            let expected = model::TypeStructure::NamedFields(vec![
                (
                    quote::format_ident!("relative_path"),
                    model::Populator::RelativePath,
                ),
                (
                    quote::format_ident!("contents_str"),
                    model::Populator::ContentsStr,
                ),
            ]);
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_nonstandard_field_it_errs() {
            let actual = main(model::TypeStructure::NamedFields(vec![
                (quote::format_ident!("relative_path"), ()),
                (quote::format_ident!("abc"), ()),
            ]));

            let actual = actual.unwrap_err();
            let expected = model::Error::NonstandardField {
                field: quote::format_ident!("abc"),
                standard_fields: vec![
                    quote::format_ident!("contents_bytes"),
                    quote::format_ident!("contents_str"),
                    quote::format_ident!("get_bytes"),
                    quote::format_ident!("get_str"),
                    quote::format_ident!("relative_path"),
                ],
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_each_standard_field() {
            let actual = main(model::TypeStructure::NamedFields(vec![
                (quote::format_ident!("contents_bytes"), ()),
                (quote::format_ident!("contents_str"), ()),
                (quote::format_ident!("get_bytes"), ()),
                (quote::format_ident!("get_str"), ()),
                (quote::format_ident!("relative_path"), ()),
            ]));

            let actual = actual.unwrap();
            let expected = model::TypeStructure::NamedFields(vec![
                (
                    quote::format_ident!("contents_bytes"),
                    model::Populator::ContentsBytes,
                ),
                (
                    quote::format_ident!("contents_str"),
                    model::Populator::ContentsStr,
                ),
                (
                    quote::format_ident!("get_bytes"),
                    model::Populator::GetBytes,
                ),
                (quote::format_ident!("get_str"), model::Populator::GetStr),
                (
                    quote::format_ident!("relative_path"),
                    model::Populator::RelativePath,
                ),
            ]);
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod handles_tuple_fields {
        use super::*;

        #[test]
        fn given_no_fields_it_handles() {
            let actual = main(model::TypeStructure::TupleFields(vec![]));

            let actual = actual.unwrap();
            let expected = model::TypeStructure::TupleFields(vec![]);
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_fields_it_errs() {
            let actual = main(model::TypeStructure::TupleFields(vec![()]));

            let actual = actual.unwrap_err();
            let expected = model::Error::NoInitializer;
            assert_eq!(actual, expected);
        }
    }
}
