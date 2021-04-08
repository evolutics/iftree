use crate::model;

pub static PREDEFINED_TEMPLATES: &[(&str, model::Template)] = &[
    ("absolute_path", model::Template::AbsolutePath),
    ("content", model::Template::Content),
    ("raw_content", model::Template::RawContent),
    ("relative_path", model::Template::RelativePath),
];
