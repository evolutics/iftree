use crate::model;

pub trait TryMap {
    type Input;
    type Output;

    fn map_unit(&self) -> model::Result<()>;

    fn map_type_alias(&self, annotation: &Self::Input) -> model::Result<Self::Output>;

    fn map_named_field(&self, name: &str, annotation: &Self::Input) -> model::Result<Self::Output>;

    fn map_tuple_field(
        &self,
        index: usize,
        annotation: &Self::Input,
    ) -> model::Result<Self::Output>;
}

pub fn main<T, U>(
    try_map: &impl TryMap<Input = T, Output = U>,
    resource: &model::AbstractResource<T>,
) -> model::Result<model::AbstractResource<U>> {
    Ok(match resource {
        model::AbstractResource::Unit => {
            try_map.map_unit()?;
            model::AbstractResource::Unit
        }

        model::AbstractResource::TypeAlias(annotation) => {
            model::AbstractResource::TypeAlias(try_map.map_type_alias(annotation)?)
        }

        model::AbstractResource::NamedFields(annotations) => model::AbstractResource::NamedFields(
            annotations
                .iter()
                .map(|(name, annotation)| {
                    Ok((name.clone(), try_map.map_named_field(name, annotation)?))
                })
                .collect::<model::Result<_>>()?,
        ),

        model::AbstractResource::TupleFields(annotations) => model::AbstractResource::TupleFields(
            annotations
                .iter()
                .enumerate()
                .map(|(index, annotation)| try_map.map_tuple_field(index, annotation))
                .collect::<model::Result<_>>()?,
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Converter;

    impl TryMap for Converter {
        type Input = char;
        type Output = char;

        fn map_unit(&self) -> model::Result<()> {
            Ok(())
        }

        fn map_type_alias(&self, annotation: &Self::Input) -> model::Result<Self::Output> {
            Ok(annotation.to_ascii_lowercase())
        }

        fn map_named_field(
            &self,
            _name: &str,
            annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            Ok(annotation.to_ascii_lowercase())
        }

        fn map_tuple_field(
            &self,
            _index: usize,
            annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            Ok(annotation.to_ascii_lowercase())
        }
    }

    #[test]
    fn tries_unit() {
        let actual = main(&Converter, &model::AbstractResource::Unit);

        let actual = actual.unwrap();
        let expected = model::AbstractResource::Unit;
        assert_eq!(actual, expected);
    }

    #[test]
    fn tries_type_alias() {
        let actual = main(&Converter, &model::AbstractResource::TypeAlias('A'));

        let actual = actual.unwrap();
        let expected = model::AbstractResource::TypeAlias('a');
        assert_eq!(actual, expected);
    }

    #[test]
    fn tries_named_fields() {
        let actual = main(
            &Converter,
            &model::AbstractResource::NamedFields(
                vec![(String::from("foo"), 'A'), (String::from("bar"), 'B')]
                    .into_iter()
                    .collect(),
            ),
        );

        let actual = actual.unwrap();
        let expected = model::AbstractResource::NamedFields(
            vec![(String::from("foo"), 'a'), (String::from("bar"), 'b')]
                .into_iter()
                .collect(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn tries_tuple_fields() {
        let actual = main(
            &Converter,
            &model::AbstractResource::TupleFields(vec!['A', 'B']),
        );

        let actual = actual.unwrap();
        let expected = model::AbstractResource::TupleFields(vec!['a', 'b']);
        assert_eq!(actual, expected);
    }
}
