use super::generate_identifier;
use super::sanitize_to_identifier;
use crate::model;
use std::path;
use std::vec;

pub fn main(canonical_paths: vec::Vec<path::PathBuf>) -> model::FileForest {
    let mut forest = model::FileForest::new();

    for canonical_path in &canonical_paths {
        let mut reverse_file_path = get_reverse_file_path(canonical_path);
        let file = get_file(canonical_path);
        add_file(&mut forest, &mut reverse_file_path, file);
    }

    forest
}

fn get_reverse_file_path(canonical_path: &path::PathBuf) -> vec::Vec<String> {
    canonical_path
        .iter()
        .rev()
        .enumerate()
        .map(|(index, name)| {
            let name = name.to_string_lossy();
            let convention = if index == 0 {
                sanitize_to_identifier::Convention::ScreamingSnakeCase
            } else {
                sanitize_to_identifier::Convention::SnakeCase
            };
            sanitize_to_identifier::main(&name, convention)
        })
        .collect()
}

fn get_file(canonical_path: &path::PathBuf) -> model::File {
    model::File {
        path: path::PathBuf::from("resources").join(canonical_path),
    }
}

fn add_file(
    parent: &mut model::FileForest,
    reverse_file_path: &mut vec::Vec<String>,
    file: model::File,
) {
    match reverse_file_path.pop() {
        None => {}

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_file_path.to_vec(), file);
                parent.insert(name, child);
            }

            Some(model::FileTree::File(colliding_file)) => {
                eprintln!(
                    "Adapting generated name due to collision with file: {}",
                    colliding_file.path.display()
                );
                let name = generate_identifier::main(&name, &|name| !parent.contains_key(name));
                reverse_file_path.push(name);
                add_file(parent, reverse_file_path, file);
            }

            Some(model::FileTree::Folder(child)) => {
                add_file(child, reverse_file_path, file);
            }
        },
    };
}

fn get_singleton_tree(reverse_file_path: vec::Vec<String>, file: model::File) -> model::FileTree {
    let mut child = model::FileTree::File(file);

    for name in reverse_file_path.into_iter() {
        let mut parent = model::FileForest::new();
        parent.insert(name, child);
        child = model::FileTree::Folder(parent);
    }

    child
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_empty_set() {
        let actual = main(vec![]);

        assert_eq!(actual, model::FileForest::new());
    }

    #[test]
    fn gets_files() {
        let canonical_paths = vec![
            path::PathBuf::from("menu.json"),
            path::PathBuf::from("translations.csv"),
        ];

        let actual = main(canonical_paths);

        let mut expected = model::FileForest::new();
        expected.insert(
            "MENU_JSON".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/menu.json"),
            }),
        );
        expected.insert(
            "TRANSLATIONS_CSV".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/translations.csv"),
            }),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_folders() {
        let canonical_paths = vec![
            path::PathBuf::from("credits.md"),
            path::PathBuf::from("world/levels/tutorial.json"),
            path::PathBuf::from("world/physical_constants.json"),
        ];

        let actual = main(canonical_paths);

        let mut levels = model::FileForest::new();
        levels.insert(
            "TUTORIAL_JSON".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/world/levels/tutorial.json"),
            }),
        );
        let mut world = model::FileForest::new();
        world.insert("levels".to_owned(), model::FileTree::Folder(levels));
        world.insert(
            "PHYSICAL_CONSTANTS_JSON".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/world/physical_constants.json"),
            }),
        );
        let mut expected = model::FileForest::new();
        expected.insert(
            "CREDITS_MD".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/credits.md"),
            }),
        );
        expected.insert("world".to_owned(), model::FileTree::Folder(world));
        assert_eq!(actual, expected);
    }

    #[test]
    fn resolves_collisions() {
        let canonical_paths = vec![
            path::PathBuf::from("credits.md"),
            path::PathBuf::from("Credits.md"),
            path::PathBuf::from("CREDITS.md"),
            path::PathBuf::from("credits.md0"),
        ];

        let actual = main(canonical_paths);

        let mut expected = model::FileForest::new();
        expected.insert(
            "CREDITS_MD".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/credits.md"),
            }),
        );
        expected.insert(
            "CREDITS_MD0".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/Credits.md"),
            }),
        );
        expected.insert(
            "CREDITS_MD1".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/CREDITS.md"),
            }),
        );
        expected.insert(
            "CREDITS_MD00".to_owned(),
            model::FileTree::File(model::File {
                path: path::PathBuf::from("resources/credits.md0"),
            }),
        );
        assert_eq!(actual, expected);
    }
}
