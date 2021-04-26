use crate::model;

pub const ASSET_ARRAY_NAME: &str = "ASSETS";

pub const BASE_MODULE_NAME: &str = "base";

pub const DEBUG_NAME: &str = "DEBUG";

pub static STANDARD_FIELD_POPULATORS_ORDERED: &[(&str, model::Populator)] = &[
    ("contents_bytes", model::Populator::ContentsBytes),
    ("contents_str", model::Populator::ContentsStr),
    ("get_bytes", model::Populator::GetBytes),
    ("get_str", model::Populator::GetStr),
    ("relative_path", model::Populator::RelativePath),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_field_populators_are_strictly_ordered() {
        for (left, right) in STANDARD_FIELD_POPULATORS_ORDERED[1..].iter().enumerate() {
            let left = &STANDARD_FIELD_POPULATORS_ORDERED[left];

            let actual = left.0 < right.0;

            assert!(actual);
        }
    }
}
