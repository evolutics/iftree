use crate::model;
use std::path;

pub fn main(resource_type: model::TypeAlias) -> model::FileIndex {
    model::FileIndex {
        resource_type: resource_type.identifier.to_string(),
        files: example_files(),
    }
}

fn example_files() -> model::FileForest {
    let menu_json = model::FileTree::File(model::File {
        path: path::PathBuf::from("resources/configuration/menu.json"),
    });
    let translations_csv = model::FileTree::File(model::File {
        path: path::PathBuf::from("resources/configuration/translations.csv"),
    });
    let credits_md = model::FileTree::File(model::File {
        path: path::PathBuf::from("resources/credits.md"),
    });
    let tutorial_json = model::FileTree::File(model::File {
        path: path::PathBuf::from("resources/world/levels/tutorial.json"),
    });
    let physical_constants_json = model::FileTree::File(model::File {
        path: path::PathBuf::from("resources/world/physical_constants.json"),
    });

    let mut configuration = model::FileForest::new();
    configuration.insert("MENU_JSON".to_owned(), menu_json);
    configuration.insert("TRANSLATIONS_CSV".to_owned(), translations_csv);
    let mut levels = model::FileForest::new();
    levels.insert("TUTORIAL_JSON".to_owned(), tutorial_json);
    let mut world = model::FileForest::new();
    world.insert("levels".to_owned(), model::FileTree::Folder(levels));
    world.insert(
        "PHYSICAL_CONSTANTS_JSON".to_owned(),
        physical_constants_json,
    );
    let mut files = model::FileForest::new();
    files.insert(
        "configuration".to_owned(),
        model::FileTree::Folder(configuration),
    );
    files.insert("CREDITS_MD".to_owned(), credits_md);
    files.insert("world".to_owned(), model::FileTree::Folder(world));

    files
}
