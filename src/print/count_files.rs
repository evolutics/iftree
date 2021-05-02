use crate::model;

pub fn main(forest: &model::FileForest) -> usize {
    forest
        .values()
        .map(|tree| match tree {
            model::FileTree::File(_) => 1,
            model::FileTree::Folder(model::Folder { forest, .. }) => main(forest),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(
            &vec![
                (
                    String::from('0'),
                    model::FileTree::File(model::stubs::file()),
                ),
                (
                    String::from('1'),
                    model::FileTree::Folder(model::Folder {
                        forest: vec![
                            (
                                String::from('2'),
                                model::FileTree::Folder(model::Folder {
                                    forest: vec![(
                                        String::from('3'),
                                        model::FileTree::File(model::stubs::file()),
                                    )]
                                    .into_iter()
                                    .collect(),
                                    ..model::stubs::folder()
                                }),
                            ),
                            (
                                String::from('4'),
                                model::FileTree::File(model::stubs::file()),
                            ),
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
