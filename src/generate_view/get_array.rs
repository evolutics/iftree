use crate::model;
use std::vec;

pub fn main(paths: vec::Vec<model::Path>) -> vec::Vec<model::Path> {
    let mut array = paths;
    array.sort_unstable_by(|left, right| left.relative.cmp(&right.relative));
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(vec![
            model::Path {
                relative: model::RelativePath::from("B"),
                absolute: String::from("one"),
            },
            model::Path {
                relative: model::RelativePath::from("A"),
                absolute: String::from("zero"),
            },
            model::Path {
                relative: model::RelativePath::from("a"),
                absolute: String::from("two"),
            },
            model::Path {
                relative: model::RelativePath::from("a/b"),
                absolute: String::from("four"),
            },
            model::Path {
                relative: model::RelativePath::from("a.c"),
                absolute: String::from("three"),
            },
        ]);

        let expected = vec![
            model::Path {
                relative: model::RelativePath::from("A"),
                absolute: String::from("zero"),
            },
            model::Path {
                relative: model::RelativePath::from("B"),
                absolute: String::from("one"),
            },
            model::Path {
                relative: model::RelativePath::from("a"),
                absolute: String::from("two"),
            },
            model::Path {
                relative: model::RelativePath::from("a.c"),
                absolute: String::from("three"),
            },
            model::Path {
                relative: model::RelativePath::from("a/b"),
                absolute: String::from("four"),
            },
        ];
        assert_eq!(actual, expected);
    }
}
