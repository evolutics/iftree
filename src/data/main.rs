use crate::model;

pub const ANONYMOUS_FIELD_IDENTIFIER: &str = "_";

pub const ASSET_ARRAY_IDENTIFIER: &str = "ASSETS";

pub const BASE_MODULE_IDENTIFIER: &str = "base";

pub static PREDEFINED_TEMPLATES_ORDERED: &[(&str, model::Template)] = &[
    ("content", model::Template::Content),
    ("get_content", model::Template::GetContent),
    ("get_raw_content", model::Template::GetRawContent),
    ("raw_content", model::Template::RawContent),
    ("relative_path", model::Template::RelativePath),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predefined_templates_are_strictly_ordered() {
        for (left, right) in PREDEFINED_TEMPLATES_ORDERED[1..].iter().enumerate() {
            let left = &PREDEFINED_TEMPLATES_ORDERED[left];

            let actual = left.0 < right.0;

            assert!(actual);
        }
    }
}
