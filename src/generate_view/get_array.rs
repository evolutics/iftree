use crate::model;
use std::vec;

pub fn main(paths: vec::Vec<model::Path>) -> vec::Vec<model::Path> {
    let mut array = paths;
    array.sort_unstable_by(|left, right| left.relative_path.cmp(&right.relative_path));
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(vec![
            model::Path {
                relative_path: model::RelativePath::from("B"),
                absolute_path: String::from("one"),
            },
            model::Path {
                relative_path: model::RelativePath::from("A"),
                absolute_path: String::from("zero"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a"),
                absolute_path: String::from("two"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: String::from("four"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a.c"),
                absolute_path: String::from("three"),
            },
        ]);

        let expected = vec![
            model::Path {
                relative_path: model::RelativePath::from("A"),
                absolute_path: String::from("zero"),
            },
            model::Path {
                relative_path: model::RelativePath::from("B"),
                absolute_path: String::from("one"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a"),
                absolute_path: String::from("two"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a.c"),
                absolute_path: String::from("three"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: String::from("four"),
            },
        ];
        assert_eq!(actual, expected);
    }
}
