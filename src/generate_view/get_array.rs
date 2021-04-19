use crate::model;
use std::vec;

pub fn main(files: vec::Vec<model::File>) -> vec::Vec<model::File> {
    let mut array = files;
    array.sort_unstable_by(|left, right| left.relative_path.cmp(&right.relative_path));
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(vec![
            model::File {
                relative_path: model::RelativePath::from("B"),
                absolute_path: String::from("one"),
            },
            model::File {
                relative_path: model::RelativePath::from("A"),
                absolute_path: String::from("zero"),
            },
            model::File {
                relative_path: model::RelativePath::from("a"),
                absolute_path: String::from("two"),
            },
            model::File {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: String::from("four"),
            },
            model::File {
                relative_path: model::RelativePath::from("a.c"),
                absolute_path: String::from("three"),
            },
        ]);

        let expected = vec![
            model::File {
                relative_path: model::RelativePath::from("A"),
                absolute_path: String::from("zero"),
            },
            model::File {
                relative_path: model::RelativePath::from("B"),
                absolute_path: String::from("one"),
            },
            model::File {
                relative_path: model::RelativePath::from("a"),
                absolute_path: String::from("two"),
            },
            model::File {
                relative_path: model::RelativePath::from("a.c"),
                absolute_path: String::from("three"),
            },
            model::File {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: String::from("four"),
            },
        ];
        assert_eq!(actual, expected);
    }
}