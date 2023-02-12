use crate::model;

pub fn main(forest: &model::Forest) -> usize {
    forest
        .values()
        .map(|tree| match tree {
            model::Tree::File(_) => 1,
            model::Tree::Folder(model::Folder { forest, .. }) => main(forest),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(
            &[
                ("0".into(), model::Tree::File(model::stubs::file())),
                (
                    "1".into(),
                    model::Tree::Folder(model::Folder {
                        forest: [
                            (
                                "2".into(),
                                model::Tree::Folder(model::Folder {
                                    forest: [("3".into(), model::Tree::File(model::stubs::file()))]
                                        .into_iter()
                                        .collect(),
                                    ..model::stubs::folder()
                                }),
                            ),
                            ("4".into(), model::Tree::File(model::stubs::file())),
                        ]
                        .into_iter()
                        .collect(),
                        ..model::stubs::folder()
                    }),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let expected = 3;
        assert_eq!(actual, expected);
    }
}
