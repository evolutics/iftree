use crate::model;

pub static PREDEFINED_TEMPLATES_ORDERED: &[(&str, model::Template)] = &[
    ("absolute_path", model::Template::AbsolutePath),
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
