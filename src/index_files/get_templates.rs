use super::try_map_abstract_resource;
use crate::data;
use crate::model;

pub fn main<'a>(
    configuration: &'a model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
) -> model::Result<model::AbstractResource<&'a model::Template>> {
    impl<'a> try_map_abstract_resource::TryMap for &'a model::Configuration {
        type Input = ();
        type Output = &'a model::Template;

        fn map_unit(&self) -> model::Result<()> {
            Ok(())
        }

        fn map_type_alias(&self, _annotation: &Self::Input) -> model::Result<Self::Output> {
            get_template(self, model::FieldIdentifier::Anonymous)
        }

        fn map_named_field(
            &self,
            name: &str,
            _annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            get_template(self, model::FieldIdentifier::Named(String::from(name)))
        }

        fn map_tuple_field(
            &self,
            index: usize,
            _annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            get_template(self, model::FieldIdentifier::Indexed(index))
        }
    }

    try_map_abstract_resource::main(&configuration, resource_structure)
}

fn get_template(
    configuration: &model::Configuration,
    identifier: model::FieldIdentifier,
) -> model::Result<&model::Template> {
    match configuration.field_templates.get(&identifier) {
        None => {
            let name = String::from(identifier.clone());
            match data::PREDEFINED_TEMPLATES_ORDERED.binary_search_by(|entry| entry.0.cmp(&name)) {
                Err(_) => Err(model::Error::MissingFieldTemplate(identifier)),
                Ok(index) => Ok(&data::PREDEFINED_TEMPLATES_ORDERED[index].1),
            }
        }

        Some(template) => Ok(template),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_field_template_at_all_it_errs() {
        let configuration = model::Configuration {
            field_templates: model::FieldTemplates::new(),
            ..model::stubs::configuration()
        };

        let actual = main(&configuration, &model::ResourceTypeStructure::TypeAlias(()));

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingFieldTemplate(model::FieldIdentifier::Anonymous);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_configured_field_template_it_defaults_to_predefined() {
        let configuration = model::Configuration {
            field_templates: model::FieldTemplates::new(),
            ..model::stubs::configuration()
        };

        let actual = main(
            &configuration,
            &model::ResourceTypeStructure::NamedFields(vec![(String::from("raw_content"), ())]),
        );

        let actual = actual.unwrap();
        let expected = model::AbstractResource::NamedFields(vec![(
            String::from("raw_content"),
            &model::Template::RawContent,
        )]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_configured_field_template_it_gets_it() {
        let configuration = model::Configuration {
            field_templates: vec![(
                model::FieldIdentifier::Named(String::from("my_content")),
                model::Template::Content,
            )]
            .into_iter()
            .collect(),
            ..model::stubs::configuration()
        };

        let actual = main(
            &configuration,
            &model::ResourceTypeStructure::NamedFields(vec![(String::from("my_content"), ())]),
        );

        let actual = actual.unwrap();
        let expected = model::AbstractResource::NamedFields(vec![(
            String::from("my_content"),
            &model::Template::Content,
        )]);
        assert_eq!(actual, expected);
    }
}
