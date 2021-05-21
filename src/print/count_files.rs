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
    use std::array;

    #[test]
    fn handles() {
        let actual = main(
            &array::IntoIter::new([
                (String::from('0'), model::Tree::File(model::stubs::file())),
                (
                    String::from('1'),
                    model::Tree::Folder(model::Folder {
                        forest: array::IntoIter::new([
                            (
                                String::from('2'),
                                model::Tree::Folder(model::Folder {
                                    forest: array::IntoIter::new([(
                                        String::from('3'),
                                        model::Tree::File(model::stubs::file()),
                                    )])
                                    .collect(),
                                    ..model::stubs::folder()
                                }),
                            ),
                            (String::from('4'), model::Tree::File(model::stubs::file())),
                        ])
                        .collect(),
                        ..model::stubs::folder()
                    }),
                ),
            ])
            .collect(),
        );

        let expected = 3;
        assert_eq!(actual, expected);
    }
}
