use crate::model;
use std::vec;

pub trait Visitor<'a> {
    type State;

    fn file(&self, file: &'a model::File, path: &[&str], state: &mut Self::State);

    fn before_forest(&self, path: &[&str], state: &mut Self::State);

    fn after_forest(&self, path: &[&str], state: &mut Self::State);
}

pub fn visit<'a, T>(
    visitor: &impl Visitor<'a, State = T>,
    forest: &'a model::FileForest,
    state: &mut T,
) {
    visit_recursively(visitor, forest, &mut vec![], state)
}

fn visit_recursively<'a, T>(
    visitor: &impl Visitor<'a, State = T>,
    forest: &'a model::FileForest,
    path: &mut vec::Vec<&'a str>,
    state: &mut T,
) {
    visitor.before_forest(path, state);

    for (name, tree) in forest {
        path.push(name);

        match tree {
            model::FileTree::File(file) => visitor.file(file, path, state),
            model::FileTree::Folder(forest) => visit_recursively(visitor, forest, path, state),
        }

        path.pop();
    }

    visitor.after_forest(path, state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    struct Indenter {
        length: usize,
    }

    impl Indenter {
        fn format(&self, level: usize, line: &str) -> String {
            format!("{}{}\n", " ".repeat(level * self.length), line)
        }
    }

    impl Visitor<'_> for Indenter {
        type State = String;

        fn file(&self, file: &model::File, path: &[&str], text: &mut Self::State) {
            let key = path.last().unwrap();
            let value = file.relative_path.to_string_lossy();
            text.push_str(&self.format(path.len(), &format!("{}: {}", key, value)));
        }

        fn before_forest(&self, path: &[&str], text: &mut Self::State) {
            let name = path.last().unwrap_or(&"");
            text.push_str(&self.format(path.len(), &format!("({}", name)));
        }

        fn after_forest(&self, path: &[&str], text: &mut Self::State) {
            text.push_str(&self.format(path.len(), ")"));
        }
    }

    #[test]
    fn visits() {
        let indenter = Indenter { length: 2 };
        let grandparent = vec![(
            String::from("Parent"),
            model::FileTree::Folder(
                vec![(
                    String::from("Child"),
                    model::FileTree::Folder(
                        vec![(
                            String::from("Grandchild"),
                            model::FileTree::File(model::File {
                                relative_path: path::PathBuf::from("abc"),
                                ..model::stubs::file()
                            }),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                )]
                .into_iter()
                .collect(),
            ),
        )]
        .into_iter()
        .collect();
        let mut actual = String::new();

        visit(&indenter, &grandparent, &mut actual);

        let expected = "(
  (Parent
    (Child
      Grandchild: abc
    )
  )
)
";
        assert_eq!(actual, expected);
    }
}
