use crate::model;

pub const ASSET_ARRAY_NAME: &str = "ASSETS";

pub const BASE_MODULE_NAME: &str = "base";

pub const DEBUG_NAME: &str = "DEBUG";

pub static STANDARD_FIELD_TEMPLATES_ORDERED: &[(&str, model::Template)] = &[
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
    fn standard_field_templates_are_strictly_ordered() {
        for (left, right) in STANDARD_FIELD_TEMPLATES_ORDERED[1..].iter().enumerate() {
            let left = &STANDARD_FIELD_TEMPLATES_ORDERED[left];

            let actual = left.0 < right.0;

            assert!(actual);
        }
    }
}
